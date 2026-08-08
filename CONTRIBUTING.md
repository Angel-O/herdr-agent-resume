# Contributing

Contributions are welcome. Keep changes focused, explain the user-facing reason
for them, and add tests for behavior changes.

## Report A Bug

Open a GitHub issue with:

- your Herdr version from `herdr --version`
- operating system and architecture
- the agent and its version
- steps to reproduce the problem
- expected and actual behavior
- relevant output from `herdr plugin log list --plugin angel-o.agent-resume`

Please search existing issues first and remove private information, session
identifiers, and other sensitive terminal content from logs.

## Development Setup

Development requires Rust 1.89 or newer and Herdr 0.7.5 or newer.

Build and link the working tree:

```bash
cargo build --release
herdr plugin link "$PWD"
```

`herdr plugin link` does not run manifest build commands. Rebuild locally after
changing Rust code.

## Make A Change

- Create a branch from `main`.
- Keep the flat module structure under `src/`; avoid nested modules for this
  small plugin.
- Keep resume-command detection pure and isolated from Herdr, environment, and
  clipboard I/O.
- Preserve newest-command precedence when adding an agent format.
- Never execute a detected command automatically or read clipboard contents.
- Add or update tests alongside the module whose behavior changes.

Do not commit generated files under `target/` or manually built release
binaries. Version bumps and release workflow changes should be isolated in a
release-focused pull request.

## Verify

Run the same checks used by CI:

```bash
cargo fmt --check
cargo test --all-targets --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items --locked
scripts/check-version.sh
scripts/test-install.sh
```

## Open A Pull Request

Describe what changed, why it is needed, how it was tested, and any behavior or
compatibility implications. Keep unrelated changes in separate pull requests.

By contributing, you agree that your contribution is licensed under the
project's [MIT License](LICENSE).
