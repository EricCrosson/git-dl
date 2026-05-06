use std::fmt::Display;
use std::process::Command;

#[derive(Debug)]
#[non_exhaustive]
pub struct ResolveTokenError {
    kind: ResolveTokenErrorKind,
}

#[derive(Debug)]
enum ResolveTokenErrorKind {
    Io(std::io::Error),
    InvalidUtf8(std::string::FromUtf8Error),
    NotFound,
}

impl ResolveTokenError {
    fn io(err: std::io::Error) -> Self {
        Self {
            kind: ResolveTokenErrorKind::Io(err),
        }
    }

    fn invalid_utf8(err: std::string::FromUtf8Error) -> Self {
        Self {
            kind: ResolveTokenErrorKind::InvalidUtf8(err),
        }
    }

    fn not_found() -> Self {
        Self {
            kind: ResolveTokenErrorKind::NotFound,
        }
    }
}

impl Display for ResolveTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "no GitHub token found; provide a token using one of these methods (in order of precedence):\n  \
            1. --github-token <TOKEN>\n  \
            2. GITHUB_TOKEN environment variable\n  \
            3. Install and authenticate the GitHub CLI: gh auth login"
        )
    }
}

impl std::error::Error for ResolveTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ResolveTokenErrorKind::Io(err) => Some(err),
            ResolveTokenErrorKind::InvalidUtf8(err) => Some(err),
            ResolveTokenErrorKind::NotFound => None,
        }
    }
}

fn parse_gh_output(success: bool, stdout: Vec<u8>) -> Result<String, ResolveTokenError> {
    if !success {
        return Err(ResolveTokenError::not_found());
    }

    let token = String::from_utf8(stdout)
        .map_err(ResolveTokenError::invalid_utf8)?
        .trim()
        .to_string();

    if token.is_empty() {
        return Err(ResolveTokenError::not_found());
    }

    Ok(token)
}

pub fn resolve_token_from_gh_cli(hostname: &str) -> Result<String, ResolveTokenError> {
    let output = Command::new("gh")
        .args(["auth", "token", "--hostname", hostname])
        .output()
        .map_err(ResolveTokenError::io)?;

    parse_gh_output(output.status.success(), output.stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gh_output_returns_trimmed_token() {
        let result = parse_gh_output(true, b"ghp_abc123\n".to_vec());
        assert_eq!(result.unwrap(), "ghp_abc123");
    }

    #[test]
    fn parse_gh_output_trims_surrounding_whitespace() {
        let result = parse_gh_output(true, b"  ghp_abc123  \n".to_vec());
        assert_eq!(result.unwrap(), "ghp_abc123");
    }

    #[test]
    fn parse_gh_output_errors_on_failed_status() {
        assert!(parse_gh_output(false, b"ghp_abc123\n".to_vec()).is_err());
    }

    #[test]
    fn parse_gh_output_errors_on_empty_stdout() {
        assert!(parse_gh_output(true, b"".to_vec()).is_err());
    }

    #[test]
    fn parse_gh_output_errors_on_whitespace_only_stdout() {
        assert!(parse_gh_output(true, b"   \n".to_vec()).is_err());
    }
}
