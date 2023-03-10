use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use crate::error::Result;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct GithubRepositoryOwner {
    pub login: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct GetRepositoryResponse {
    pub name: String,
    pub owner: GithubRepositoryOwner,
}

#[derive(Clone, Debug)]
pub(crate) struct Repo {
    pub owner: String,
    pub name: String,
}

impl Repo {
    pub(crate) fn directory(&self, home: &Path) -> PathBuf {
        home.join("workspace").join(&self.owner).join(&self.name)
    }

    pub(crate) fn clone(&self, home: &Path) -> Result<()> {
        let target_directory = self.directory(home);
        Command::new("git")
            .arg("clone")
            .arg(self.to_string())
            .arg(target_directory)
            .status()?;
        Ok(())
    }
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "git@github.com:{}/{}.git", self.owner, self.name)
    }
}

impl FromStr for Repo {
    type Err = Box<dyn Error + Send + Sync + 'static>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let shortened = s
            .trim()
            .replace("https://", "")
            .replace("git@github.com:", "")
            .replace(".git", "");
        let pattern = shortened.split_once('/');
        if let Some((owner, repository)) = pattern {
            return Ok(Self {
                owner: owner.to_owned(),
                name: repository.to_owned(),
            });
        } else {
            return Err(format!("Unrecognized repository format: {}", s))?;
        }
    }
}

impl From<GetRepositoryResponse> for Repo {
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
