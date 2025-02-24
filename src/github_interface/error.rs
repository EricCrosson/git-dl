use std::fmt::Display;

#[derive(Debug)]
#[non_exhaustive]
pub struct GithubError {
    kind: GithubErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

#[derive(Debug)]
pub enum GithubErrorKind {
    RequestFailed,
    InvalidResponse,
    ParseError,
}

impl GithubError {
    pub(crate) fn request_failed(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            kind: GithubErrorKind::RequestFailed,
            source: Some(Box::new(source)),
        }
    }

    pub(crate) fn invalid_response(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            kind: GithubErrorKind::InvalidResponse,
            source: Some(Box::new(source)),
        }
    }

    pub(crate) fn parse_error(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            kind: GithubErrorKind::ParseError,
            source: Some(Box::new(source)),
        }
    }
}

impl Display for GithubError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            GithubErrorKind::RequestFailed => write!(f, "GitHub API request failed"),
            GithubErrorKind::InvalidResponse => write!(f, "invalid response from GitHub API"),
            GithubErrorKind::ParseError => write!(f, "failed to parse GitHub API response"),
        }
    }
}

impl std::error::Error for GithubError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}
