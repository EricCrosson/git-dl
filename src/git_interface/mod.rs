pub mod error;
pub mod git;

use self::error::GitError;
use std::path::Path;

pub trait GitOperations {
    fn clone(&self, url: &str, target: &Path) -> Result<(), GitError>;
    fn add_remote(&self, repo_path: &Path, name: &str, url: &str) -> Result<(), GitError>;
}
