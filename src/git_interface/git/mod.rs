use crate::git_interface::{error::GitError, GitOperations};
use std::path::Path;
use std::process::Command;

pub struct SystemGit;

impl SystemGit {
    pub fn new() -> Self {
        Self
    }
}

impl GitOperations for SystemGit {
    fn clone(&self, url: &str, target: &Path) -> Result<(), GitError> {
        Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(target)
            .status()
            .map_err(GitError::clone_failed)?;
        Ok(())
    }

    fn add_remote(&self, repo_path: &Path, name: &str, url: &str) -> Result<(), GitError> {
        Command::new("git")
            .arg("remote")
            .arg("add")
            .arg(name)
            .arg(url)
            .current_dir(repo_path)
            .status()
            .map_err(GitError::remote_add_failed)?;
        Ok(())
    }
}
