//! Herdr plugin that inserts or copies agent resume commands from pane output.

mod clipboard;
mod herdr;
mod invocation;
mod resume;

use invocation::{Action, Invocation};

fn main() {
    if let Err(error) = run() {
        eprintln!("herdr-agent-resume: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let invocation = Invocation::from_env()?;
    let pane_output = herdr::read_recent_output(&invocation.herdr, &invocation.pane_id)?;
    let resume_command = resume::latest_command(&pane_output).ok_or_else(|| {
        "no supported agent resume command was found in recent pane output".to_string()
    })?;

    match invocation.action {
        Action::Insert => {
            herdr::insert_text(&invocation.herdr, &invocation.pane_id, resume_command)?;
            println!("inserted agent resume command");
        }
        Action::Copy => {
            clipboard::copy(resume_command)?;
            println!("copied agent resume command");
        }
    }

    Ok(())
}
