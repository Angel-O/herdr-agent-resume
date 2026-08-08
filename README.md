# Herdr Agent Resume

A Herdr plugin that inserts or copies the latest agent resume command printed in
the focused pane after an agent exits. OpenCode, Codex, Claude Code, and Factory
Droid are currently supported.

The plugin reads recent unwrapped pane output and copies the newest command that
matches a supported agent's exit message:

```text
opencode -s ses_...
codex resume 019f88e3-...
claude --resume b3dbde41-...
droid --resume droid-session-id
```

It does not depend on the cursor position, a fixed terminal layout, or Herdr's
agent metadata. If commands from multiple agents are present, the command that
appears latest in the pane output is selected.

## Requirements

- Herdr 0.7.5 or newer
- Rust 1.89 or newer
- macOS, or Linux with `wl-copy`, `xclip`, or `xsel` for the copy action

## Local Setup

Build and link the plugin from this directory:

```bash
cargo build --release
herdr plugin link "$PWD"
```

Add the keybindings to `~/.config/herdr/config.toml`:

```toml
[[keys.command]]
key = "prefix+a"
type = "plugin_action"
command = "angel-o.agent-resume.insert-resume"
description = "insert agent resume command"

[[keys.command]]
key = "prefix+shift+a"
type = "plugin_action"
command = "angel-o.agent-resume.copy-resume"
description = "copy agent resume command"
```

Reload the configuration:

```bash
herdr server reload-config
```

After exiting a supported agent, press the Herdr prefix followed by `A` to insert
the full command at the shell prompt without executing it. Use `prefix+shift+a`
to copy it to the clipboard instead.

## Development

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Remote Sessions

Direct insertion works in remote sessions because Herdr sends the text to the
server-side pane. Herdr does not currently expose clipboard writes to plugins,
so the copy action writes to the clipboard on the machine running the Herdr
server rather than the attaching computer.
