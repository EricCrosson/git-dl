use crate::github_interface::GetRepositoryResponse;
pub mod error;
use error::ParseError;
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub(crate) struct Repo {
    pub owner: String,
    pub name: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct CloneError {
    kind: CloneErrorKind,
}

impl Display for CloneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            CloneErrorKind::Exec(_) => write!(f, "unable to clone repository"),
        }
    }
}

impl Error for CloneError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            CloneErrorKind::Exec(err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub enum CloneErrorKind {
    #[non_exhaustive]
    Exec(io::Error),
}

impl From<io::Error> for CloneError {
    fn from(err: io::Error) -> Self {
        Self {
            kind: CloneErrorKind::Exec(err),
        }
    }
}

impl Repo {
    pub(crate) fn directory(&self, home: &Path) -> PathBuf {
        home.join("workspace").join(&self.owner).join(&self.name)
    }
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "git@github.com:{}/{}.git", self.owner, self.name)
    }
}

impl FromStr for Repo {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let shortened = s
            .trim()
            .replace("https://", "")
            .replace("git@github.com:", "")
            .replace(".git", "");
        let pattern = shortened.split_once('/');
        if let Some((owner, repository)) = pattern {
            Ok(Self {
                owner: owner.to_owned(),
                name: repository.to_owned(),
            })
        } else {
            Err(ParseError::invalid_format(s))
        }
    }
}

impl From<crate::github_interface::GetRepositoryResponse> for Repo {
    fn from(value: GetRepositoryResponse) -> Self {
        Self {
            owner: value.owner.login,
            name: value.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Repo;

    fn check(input: &str, expected_owner: &str, expected_repository: &str) {
        let actual = Repo::from_str(input).unwrap();
        assert_eq!(expected_owner, actual.owner);
        assert_eq!(expected_repository, actual.name);
    }

    #[test]
    fn should_parse_git_protocol() {
        check(
            "git@github.com:EricCrosson/git-dl.git",
            "EricCrosson",
            "git-dl",
        );
    }

    #[test]
    fn should_parse_git_protocol_without_suffix() {
        check("git@github.com:EricCrosson/git-dl", "EricCrosson", "git-dl");
    }

    #[test]
    fn should_parse_https_protocol() {
        check("https://EricCrosson/git-dl", "EricCrosson", "git-dl");
    }

    #[test]
    fn should_parse_https_protocol_without_suffix() {
        check("https://EricCrosson/git-dl.git", "EricCrosson", "git-dl");
    }

    #[test]
    fn should_parse_slang() {
        check("EricCrosson/git-dl", "EricCrosson", "git-dl");
    }
}
