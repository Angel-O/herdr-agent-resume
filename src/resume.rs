//! Pure detection policy for supported agent resume commands.

const COMMAND_PREFIXES: [&str; 4] = [
    "opencode -s ses_",
    "codex resume ",
    "claude --resume ",
    "droid --resume ",
];

pub(crate) fn latest_command(output: &str) -> Option<&str> {
    COMMAND_PREFIXES
        .iter()
        .filter_map(|prefix| latest_command_with_prefix(output, prefix))
        .max_by_key(|(start, _)| *start)
        .map(|(_, command)| command)
}

fn latest_command_with_prefix<'a>(output: &'a str, prefix: &str) -> Option<(usize, &'a str)> {
    output.rmatch_indices(prefix).find_map(|(start, _)| {
        let session_start = start + prefix.len();
        let session_len = output[session_start..]
            .bytes()
            .take_while(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
            .count();

        (session_len > 0).then(|| (start, &output[start..session_start + session_len]))
    })
}

#[cfg(test)]
mod tests {
    use super::latest_command;

    #[test]
    fn extracts_opencode_command_from_exit_screen() {
        let output = "Session  Herdr navigation plugin comparison\n\
                      Continue opencode -s ses_0221956a4ffeHQolAvyWwmrEgN\n$ ";

        assert_eq!(
            latest_command(output),
            Some("opencode -s ses_0221956a4ffeHQolAvyWwmrEgN")
        );
    }

    #[test]
    fn extracts_codex_command_from_exit_screen() {
        let output =
            "To continue this session, run codex resume 019f88e3-0fa1-7851-a94b-b895e299bcd6\n$ ";

        assert_eq!(
            latest_command(output),
            Some("codex resume 019f88e3-0fa1-7851-a94b-b895e299bcd6")
        );
    }

    #[test]
    fn extracts_claude_command_from_exit_screen() {
        let output = "Resume this session with:\n\
                      claude --resume b3dbde41-b5d1-49d9-899b-4c70dbd39b88\n$ ";

        assert_eq!(
            latest_command(output),
            Some("claude --resume b3dbde41-b5d1-49d9-899b-4c70dbd39b88")
        );
    }

    #[test]
    fn extracts_droid_command_from_exit_screen() {
        let output = "Resume this session with:\n\
                      droid --resume droid-session-123\n$ ";

        assert_eq!(
            latest_command(output),
            Some("droid --resume droid-session-123")
        );
    }

    #[test]
    fn chooses_opencode_when_it_is_the_latest_agent() {
        let output = "codex resume 019-old\ntext\nopencode -s ses_second";

        assert_eq!(latest_command(output), Some("opencode -s ses_second"));
    }

    #[test]
    fn chooses_codex_when_it_is_the_latest_agent() {
        let output = "opencode -s ses_first\ntext\ncodex resume 019-new";

        assert_eq!(latest_command(output), Some("codex resume 019-new"));
    }

    #[test]
    fn chooses_claude_when_it_is_the_latest_agent() {
        let output = "opencode -s ses_first\ncodex resume 019-second\nclaude --resume claude-third";

        assert_eq!(latest_command(output), Some("claude --resume claude-third"));
    }

    #[test]
    fn chooses_droid_when_it_is_the_latest_agent() {
        let output =
            "claude --resume claude-first\ncodex resume 019-second\ndroid --resume droid-third";

        assert_eq!(latest_command(output), Some("droid --resume droid-third"));
    }

    #[test]
    fn chooses_the_latest_opencode_session() {
        let output = "opencode -s ses_old\ntext\nopencode -s ses_new";

        assert_eq!(latest_command(output), Some("opencode -s ses_new"));
    }

    #[test]
    fn chooses_the_latest_codex_session() {
        let output = "codex resume 019-old\ntext\ncodex resume 019-new";

        assert_eq!(latest_command(output), Some("codex resume 019-new"));
    }

    #[test]
    fn chooses_the_latest_claude_session() {
        let output = "claude --resume claude-old\ntext\nclaude --resume claude-new";

        assert_eq!(latest_command(output), Some("claude --resume claude-new"));
    }

    #[test]
    fn chooses_the_latest_droid_session() {
        let output = "droid --resume droid-old\ntext\ndroid --resume droid-new";

        assert_eq!(latest_command(output), Some("droid --resume droid-new"));
    }

    #[test]
    fn stops_at_characters_outside_the_session_id() {
        let output = "Run `opencode -s ses_abc-123`, then continue.";

        assert_eq!(latest_command(output), Some("opencode -s ses_abc-123"));
    }

    #[test]
    fn ignores_an_empty_session_id() {
        assert_eq!(latest_command("opencode -s ses_"), None);
    }

    #[test]
    fn returns_none_when_no_resume_command_exists() {
        assert_eq!(latest_command("ordinary terminal output"), None);
    }
}
