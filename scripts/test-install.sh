#!/bin/sh

set -eu

root=$(CDPATH= cd "$(dirname "$0")/.." && pwd)
version=$("$root/scripts/check-version.sh")
"$root/scripts/check-version.sh" "v$version" >/dev/null
if "$root/scripts/check-version.sh" "$version" >/dev/null 2>&1; then
  printf '%s\n' "an unprefixed version tag must fail" >&2
  exit 1
fi
if "$root/scripts/check-version.sh" v9.9.9 >/dev/null 2>&1; then
  printf '%s\n' "a mismatched version tag must fail" >&2
  exit 1
fi
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
release_dir="$tmp/releases/v$version"
mkdir -p "$release_dir"

targets="
aarch64-apple-darwin
x86_64-apple-darwin
aarch64-unknown-linux-musl
x86_64-unknown-linux-musl
"

for target in $targets; do
  asset="herdr-agent-resume-$target"
  printf '#!/bin/sh\nprintf "%%s\\n" "%s"\n' "$target" > "$release_dir/$asset"
  if command -v sha256sum >/dev/null 2>&1; then
    hash=$(sha256sum "$release_dir/$asset" | awk '{print $1}')
  else
    hash=$(shasum -a 256 "$release_dir/$asset" | awk '{print $1}')
  fi
  printf '%s  %s\n' "$hash" "$asset" >> "$release_dir/SHA256SUMS"
done

run_case() {
  os=$1
  arch=$2
  expected=$3
  destination="$tmp/install/$os-$arch"
  HAR_OS="$os" \
    HAR_ARCH="$arch" \
    HAR_RELEASE_BASE_URL="file://$tmp/releases" \
    HAR_INSTALL_PATH="$destination" \
    "$root/scripts/install.sh" >/dev/null
  [ -x "$destination" ] || { printf '%s\n' "$os/$arch was not installed as executable" >&2; exit 1; }
  [ "$("$destination")" = "$expected" ] || { printf '%s\n' "$os/$arch installed wrong content" >&2; exit 1; }
}

run_case Darwin arm64 aarch64-apple-darwin
run_case Darwin aarch64 aarch64-apple-darwin
run_case Darwin x86_64 x86_64-apple-darwin
run_case Darwin amd64 x86_64-apple-darwin
run_case Linux aarch64 aarch64-unknown-linux-musl
run_case Linux arm64 aarch64-unknown-linux-musl
run_case Linux x86_64 x86_64-unknown-linux-musl
run_case Linux amd64 x86_64-unknown-linux-musl

if HAR_OS=Plan9 HAR_ARCH=mips HAR_RELEASE_BASE_URL="file://$tmp/releases" \
  HAR_INSTALL_PATH="$tmp/unsupported" "$root/scripts/install.sh" >/dev/null 2>&1; then
  printf '%s\n' "unsupported platforms must fail" >&2
  exit 1
fi

replacement="$tmp/replacement"
printf 'old\n' > "$replacement"
HAR_OS=Linux HAR_ARCH=x86_64 HAR_RELEASE_BASE_URL="file://$tmp/releases" \
  HAR_INSTALL_PATH="$replacement" "$root/scripts/install.sh" >/dev/null
[ "$("$replacement")" = "x86_64-unknown-linux-musl" ] || {
  printf '%s\n' "an existing binary was not replaced" >&2
  exit 1
}

assert_preserved() {
  destination=$1
  message=$2
  [ "$(cat "$destination")" = "keep" ] || {
    printf '%s\n' "$message" >&2
    exit 1
  }
}

cp "$release_dir/SHA256SUMS" "$tmp/checksums"
printf '0' >> "$release_dir/herdr-agent-resume-x86_64-unknown-linux-musl"
printf 'keep\n' > "$tmp/corrupt"
if HAR_OS=Linux HAR_ARCH=x86_64 HAR_RELEASE_BASE_URL="file://$tmp/releases" \
  HAR_INSTALL_PATH="$tmp/corrupt" "$root/scripts/install.sh" >/dev/null 2>&1; then
  printf '%s\n' "a checksum mismatch must fail" >&2
  exit 1
fi
assert_preserved "$tmp/corrupt" "a checksum failure replaced the existing binary"
mv "$tmp/checksums" "$release_dir/SHA256SUMS"

printf 'keep\n' > "$tmp/missing-binary"
if HAR_OS=Linux HAR_ARCH=x86_64 HAR_RELEASE_BASE_URL="file://$tmp/missing" \
  HAR_INSTALL_PATH="$tmp/missing-binary" "$root/scripts/install.sh" >/dev/null 2>&1; then
  printf '%s\n' "a missing binary download must fail" >&2
  exit 1
fi
assert_preserved "$tmp/missing-binary" "a missing binary replaced the existing binary"

mkdir -p "$tmp/missing-checksums/v$version"
cp "$release_dir/herdr-agent-resume-x86_64-unknown-linux-musl" "$tmp/missing-checksums/v$version/"
printf 'keep\n' > "$tmp/missing-checksums-destination"
if HAR_OS=Linux HAR_ARCH=x86_64 HAR_RELEASE_BASE_URL="file://$tmp/missing-checksums" \
  HAR_INSTALL_PATH="$tmp/missing-checksums-destination" "$root/scripts/install.sh" >/dev/null 2>&1; then
  printf '%s\n' "a missing checksum download must fail" >&2
  exit 1
fi
assert_preserved "$tmp/missing-checksums-destination" "a missing checksum replaced the existing binary"

mkdir -p "$tmp/missing-entry/v$version"
cp "$release_dir/herdr-agent-resume-x86_64-unknown-linux-musl" "$tmp/missing-entry/v$version/"
printf '%s  %s\n' "0000000000000000000000000000000000000000000000000000000000000000" \
  "another-asset" > "$tmp/missing-entry/v$version/SHA256SUMS"
printf 'keep\n' > "$tmp/missing-entry-destination"
if HAR_OS=Linux HAR_ARCH=x86_64 HAR_RELEASE_BASE_URL="file://$tmp/missing-entry" \
  HAR_INSTALL_PATH="$tmp/missing-entry-destination" "$root/scripts/install.sh" >/dev/null 2>&1; then
  printf '%s\n' "a missing checksum entry must fail" >&2
  exit 1
fi
assert_preserved "$tmp/missing-entry-destination" "a missing checksum entry replaced the existing binary"

link_tools() {
  directory=$1
  shift
  mkdir -p "$directory"
  for tool in "$@"; do
    ln -s "$(command -v "$tool")" "$directory/$tool"
  done
}

link_tools "$tmp/no-downloader" dirname sed head mktemp rm
printf 'keep\n' > "$tmp/no-downloader-destination"
if PATH="$tmp/no-downloader" HAR_OS=Linux HAR_ARCH=x86_64 \
  HAR_RELEASE_BASE_URL="file://$tmp/releases" HAR_INSTALL_PATH="$tmp/no-downloader-destination" \
  "$root/scripts/install.sh" >/dev/null 2>"$tmp/no-downloader-error"; then
  printf '%s\n' "a missing downloader must fail" >&2
  exit 1
fi
case $(cat "$tmp/no-downloader-error") in
  *"curl or wget is required"*) ;;
  *) printf '%s\n' "missing downloader error was unclear" >&2; exit 1 ;;
esac
assert_preserved "$tmp/no-downloader-destination" "a missing downloader replaced the existing binary"

link_tools "$tmp/no-checksum" dirname sed head mktemp rm curl awk
printf 'keep\n' > "$tmp/no-checksum-destination"
if PATH="$tmp/no-checksum" HAR_OS=Linux HAR_ARCH=x86_64 \
  HAR_RELEASE_BASE_URL="file://$tmp/releases" HAR_INSTALL_PATH="$tmp/no-checksum-destination" \
  "$root/scripts/install.sh" >/dev/null 2>"$tmp/no-checksum-error"; then
  printf '%s\n' "missing checksum tools must fail" >&2
  exit 1
fi
case $(cat "$tmp/no-checksum-error") in
  *"sha256sum or shasum is required"*) ;;
  *) printf '%s\n' "missing checksum tool error was unclear" >&2; exit 1 ;;
esac
assert_preserved "$tmp/no-checksum-destination" "missing checksum tools replaced the existing binary"

link_tools "$tmp/fallback-tools" dirname sed head mktemp rm awk shasum mkdir cp chmod mv
cat > "$tmp/fallback-tools/wget" <<'EOF'
#!/bin/sh
[ "$1" = "-q" ] && [ "$2" = "-O" ] || exit 2
cp "${4#file://}" "$3"
EOF
chmod 0755 "$tmp/fallback-tools/wget"
fallback_destination="$tmp/fallback-destination"
PATH="$tmp/fallback-tools" HAR_OS=Darwin HAR_ARCH=arm64 \
  HAR_RELEASE_BASE_URL="file://$tmp/releases" HAR_INSTALL_PATH="$fallback_destination" \
  "$root/scripts/install.sh" >/dev/null
[ -x "$fallback_destination" ] || {
  printf '%s\n' "the wget/shasum fallback was not installed as executable" >&2
  exit 1
}
[ "$("$fallback_destination")" = "aarch64-apple-darwin" ] || {
  printf '%s\n' "the wget/shasum fallback installed wrong content" >&2
  exit 1
}

printf '%s\n' "installer tests passed"
