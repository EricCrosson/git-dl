use std::env;
use std::fmt::Display;

use crate::repo;

#[derive(Debug)]
#[non_exhaustive]
pub struct CliError {
    kind: CliErrorKind,
    source: Option<CliErrorSource>,
}

#[derive(Debug)]
enum CliErrorKind {
    MissingRepository,
    InvalidRepository,
    EnvVarNotFound,
}

#[derive(Debug)]
enum CliErrorSource {
    EnvVar(env::VarError),
    Parse(repo::error::ParseError),
}

impl CliError {
    pub(crate) fn missing_repository() -> Self {
        Self {
            kind: CliErrorKind::MissingRepository,
            source: None,
        }
    }

    pub(crate) fn invalid_repository(source: repo::error::ParseError) -> Self {
        Self {
            kind: CliErrorKind::InvalidRepository,
            source: Some(CliErrorSource::Parse(source)),
        }
    }

    pub(crate) fn env_var_not_found(source: env::VarError) -> Self {
        Self {
            kind: CliErrorKind::EnvVarNotFound,
            source: Some(CliErrorSource::EnvVar(source)),
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            CliErrorKind::MissingRepository => write!(f, "repository argument is required"),
            CliErrorKind::InvalidRepository => write!(f, "invalid repository format"),
            CliErrorKind::EnvVarNotFound => write!(f, "environment variable not found"),
        }
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(source) => match source {
                CliErrorSource::EnvVar(err) => Some(err),
                CliErrorSource::Parse(err) => Some(err),
            },
            None => None,
        }
    }
}

impl From<repo::error::ParseError> for CliError {
    fn from(err: repo::error::ParseError) -> Self {
        Self::invalid_repository(err)
    }
}

impl From<env::VarError> for CliError {
    fn from(err: env::VarError) -> Self {
        Self::env_var_not_found(err)
    }
}
