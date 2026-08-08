//! Platform clipboard adapters used by the copy action.

use std::io::Write;
use std::process::{Command, Stdio};

#[cfg(target_os = "macos")]
pub(crate) fn copy(content: &str) -> Result<(), String> {
    write_with("pbcopy", &[], content)
}

#[cfg(target_os = "linux")]
pub(crate) fn copy(content: &str) -> Result<(), String> {
    let candidates: [(&str, &[&str]); 3] = [
        ("wl-copy", &[]),
        ("xclip", &["-selection", "clipboard"]),
        ("xsel", &["--clipboard", "--input"]),
    ];
    let mut errors = Vec::new();

    for (program, args) in candidates {
        match write_with(program, args, content) {
            Ok(()) => return Ok(()),
            Err(error) => errors.push(error),
        }
    }

    Err(format!(
        "no supported clipboard command succeeded: {}",
        errors.join("; ")
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub(crate) fn copy(_content: &str) -> Result<(), String> {
    Err("clipboard support is available only on macOS and Linux".to_string())
}

fn write_with(program: &str, args: &[&str], content: &str) -> Result<(), String> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("could not start {program}: {error}"))?;

    child
        .stdin
        .take()
        .ok_or_else(|| format!("could not open {program} stdin"))?
        .write_all(content.as_bytes())
        .map_err(|error| format!("could not write to {program}: {error}"))?;

    let output = child
        .wait_with_output()
        .map_err(|error| format!("could not wait for {program}: {error}"))?;
    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(format!("{program} failed: {}", stderr.trim()))
}
