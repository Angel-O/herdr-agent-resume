# Releasing

Releases publish prebuilt binaries for Intel and ARM macOS and Linux. Linux
artifacts use MUSL so one binary per architecture works across common
distributions.

## Prepare

1. Update the version in `Cargo.toml` and `herdr-plugin.toml`, run a Cargo
   command to refresh `Cargo.lock`, and commit the changes on a release branch.
2. Run the complete local verification suite:

   ```bash
   cargo fmt --check
   cargo test --all-targets --locked
   cargo clippy --all-targets --all-features --locked -- -D warnings
   RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items --locked
   scripts/check-version.sh
   scripts/test-install.sh
   ```

3. Open and review the release pull request. Wait for every test, quality, and
   release-target check to pass, then merge it with a merge commit so the
   reviewed release commit remains in `main` history. Do not squash the release
   pull request.

## Publish Assets

Update local `main`, then create and push an annotated version tag from the
release pull request's merge commit:

```bash
set -euo pipefail
git switch main
git pull --ff-only origin main
merge_commit="$(gh pr view <release-pr-number> --json mergeCommit --jq '.mergeCommit.oid')"
test "$(git rev-parse HEAD)" = "$merge_commit"
version="$(scripts/check-version.sh)"
git tag -a "v$version" "$merge_commit" -m "v$version"
git push origin "v$version"
```

The release workflow validates the tag, creates or reuses only a draft release,
builds all four targets, and publishes only after every build succeeds. If any
target fails, the release remains a draft.

## Verify

1. Confirm the release contains four binaries plus `SHA256SUMS`.
2. Install the reviewed tag into a clean Herdr plugin registry:

   ```bash
   herdr plugin install Angel-O/herdr-agent-resume --ref "v$version"
   ```

3. Configure both actions and verify detection, insertion, and copying for each
   available agent. Confirm failures leave an existing installed binary intact.
