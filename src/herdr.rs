//! CLI adapter for the subset of Herdr pane operations used by the plugin.

use std::process::Command;

const RECENT_OUTPUT_LINES: &str = "200";

pub(crate) fn read_recent_output(herdr: &str, pane_id: &str) -> Result<String, String> {
    let output = Command::new(herdr)
        .args([
            "pane",
            "read",
            pane_id,
            "--source",
            "recent-unwrapped",
            "--lines",
            RECENT_OUTPUT_LINES,
            "--format",
            "text",
        ])
        .output()
        .map_err(|error| format!("could not read pane {pane_id}: {error}"))?;

    if !output.status.success() {
        return Err(command_error("read", pane_id, &output.stderr));
    }

    String::from_utf8(output.stdout)
        .map_err(|_| format!("pane {pane_id} output was not valid UTF-8"))
}

pub(crate) fn insert_text(herdr: &str, pane_id: &str, content: &str) -> Result<(), String> {
    let output = Command::new(herdr)
        .args(["pane", "send-text", pane_id, content])
        .output()
        .map_err(|error| format!("could not insert command into pane {pane_id}: {error}"))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(command_error(
            "insert command into",
            pane_id,
            &output.stderr,
        ))
    }
}

fn command_error(operation: &str, pane_id: &str, stderr: &[u8]) -> String {
    format!(
        "could not {operation} pane {pane_id}: {}",
        String::from_utf8_lossy(stderr).trim()
    )
}
