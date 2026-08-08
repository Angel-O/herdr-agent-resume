#!/bin/sh

set -eu

root=$(CDPATH= cd "$(dirname "$0")/.." && pwd)
cargo_version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$root/Cargo.toml" | head -n 1)
manifest_version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$root/herdr-plugin.toml" | head -n 1)

[ -n "$cargo_version" ] || { printf '%s\n' "could not read Cargo.toml version" >&2; exit 1; }
[ -n "$manifest_version" ] || { printf '%s\n' "could not read herdr-plugin.toml version" >&2; exit 1; }
[ "$cargo_version" = "$manifest_version" ] || {
  printf 'version mismatch: Cargo.toml=%s herdr-plugin.toml=%s\n' "$cargo_version" "$manifest_version" >&2
  exit 1
}

if [ "$#" -gt 1 ]; then
  printf '%s\n' "usage: scripts/check-version.sh [vVERSION]" >&2
  exit 2
fi

if [ "$#" -eq 1 ]; then
  [ "$1" = "v$cargo_version" ] || {
    printf 'version mismatch: tag=%s source=%s\n' "$1" "$cargo_version" >&2
    exit 1
  }
fi

printf '%s\n' "$cargo_version"
