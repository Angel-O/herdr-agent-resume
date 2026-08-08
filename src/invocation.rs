//! Runtime context supplied by Herdr for one plugin action invocation.

use std::env;

const ACTION_ID_ENV: &str = "HERDR_PLUGIN_ACTION_ID";
const HERDR_BIN_ENV: &str = "HERDR_BIN_PATH";
const PANE_ID_ENV: &str = "HERDR_PANE_ID";

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Action {
    Insert,
    Copy,
}

pub(crate) struct Invocation {
    pub(crate) action: Action,
    pub(crate) herdr: String,
    pub(crate) pane_id: String,
}

impl Invocation {
    pub(crate) fn from_env() -> Result<Self, String> {
        let action_id = env::var(ACTION_ID_ENV)
            .map_err(|_| format!("{ACTION_ID_ENV} is unavailable; invoke a plugin action"))?;
        let pane_id = env::var(PANE_ID_ENV)
            .map_err(|_| format!("{PANE_ID_ENV} is unavailable; invoke this action from a pane"))?;
        let herdr = env::var(HERDR_BIN_ENV).unwrap_or_else(|_| "herdr".to_string());

        Ok(Self {
            action: parse_action(&action_id)?,
            herdr,
            pane_id,
        })
    }
}

fn parse_action(action_id: &str) -> Result<Action, String> {
    match action_id {
        "insert-resume" => Ok(Action::Insert),
        "copy-resume" => Ok(Action::Copy),
        _ => Err(format!("unsupported plugin action: {action_id}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_actions() {
        assert_eq!(parse_action("insert-resume"), Ok(Action::Insert));
        assert_eq!(parse_action("copy-resume"), Ok(Action::Copy));
    }

    #[test]
    fn rejects_unknown_actions() {
        assert_eq!(
            parse_action("run-resume"),
            Err("unsupported plugin action: run-resume".to_string())
        );
    }
}
