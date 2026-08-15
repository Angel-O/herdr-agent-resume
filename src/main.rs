//! Herdr plugin that inserts or copies agent resume commands from pane output.

mod clipboard;
mod herdr;
mod invocation;
mod resume;

use invocation::{Action, Invocation};

const OUTPUT_LINE_WINDOWS: [u32; 4] = [200, 1_000, 10_000, u32::MAX];

fn main() {
    if let Err(error) = run() {
        eprintln!("herdr-agent-resume: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let invocation = Invocation::from_env()?;
    let resume_command = find_resume_command(|lines| {
        herdr::read_recent_output(&invocation.herdr, &invocation.pane_id, lines)
    })?
    .ok_or_else(|| {
        "no supported agent resume command was found in retained pane scrollback".to_string()
    })?;

    match invocation.action {
        Action::Insert => {
            herdr::insert_text(&invocation.herdr, &invocation.pane_id, &resume_command)?;
            println!("inserted agent resume command");
        }
        Action::Copy => {
            clipboard::copy(&resume_command)?;
            println!("copied agent resume command");
        }
    }

    Ok(())
}

fn find_resume_command(
    mut read_output: impl FnMut(u32) -> Result<String, String>,
) -> Result<Option<String>, String> {
    for lines in OUTPUT_LINE_WINDOWS {
        let output = read_output(lines)?;
        if let Some(command) = resume::latest_command(&output) {
            return Ok(Some(command.to_string()));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::{OUTPUT_LINE_WINDOWS, find_resume_command};

    #[test]
    fn stops_reading_when_a_command_is_found() {
        let mut requested = Vec::new();

        let command = find_resume_command(|lines| {
            requested.push(lines);
            Ok(if lines < 1_000 {
                "ordinary terminal output".to_string()
            } else {
                "codex resume 019-found".to_string()
            })
        })
        .unwrap();

        assert_eq!(command.as_deref(), Some("codex resume 019-found"));
        assert_eq!(requested, [200, 1_000]);
    }

    #[test]
    fn reads_all_retained_scrollback_last_when_no_command_is_found() {
        let mut requested = Vec::new();

        let command = find_resume_command(|lines| {
            requested.push(lines);
            Ok("ordinary terminal output".to_string())
        })
        .unwrap();

        assert_eq!(command, None);
        assert_eq!(requested, OUTPUT_LINE_WINDOWS);
        assert_eq!(requested.last(), Some(&u32::MAX));
    }
}
