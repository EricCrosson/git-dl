pub mod error;

use std::env;
use std::path::PathBuf;

use self::error::CliError;
use crate::repo::Repo;

const HELP_TEXT: &str = "\
Clone a git repository to a structured directory

Usage: git-dl [OPTIONS] <REPO>

Arguments:
  <REPO>  Repository to clone

Options:
      --fork                         Create a fork of the target repository
      --github-token <GITHUB_TOKEN>  GitHub API token with `repo` permissions [env: GITHUB_TOKEN]
      --home <HOME>                  Location of the user's home directory [env: HOME]
  -h, --help                         Print help
  -V, --version                      Print version
";

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Args {
    pub repo: Repo,
    pub fork: bool,
    pub github_token: String,
    pub home: PathBuf,
}

impl Args {
    pub fn parse() -> Result<Self, error::CliError> {
        let args: Vec<String> = std::env::args().collect();

        // Handle help and version flags first
        if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
            println!("{}", HELP_TEXT);
            std::process::exit(0);
        }

        if args.len() > 1 && (args[1] == "-V" || args[1] == "--version") {
            println!("{}", VERSION);
            std::process::exit(0);
        }

        // Must have at least one argument (the repository)
        if args.len() < 2 {
            println!("{}", HELP_TEXT);
            return Err(CliError::missing_repository());
        }

        // Parse repository (first non-flag argument)
        let repo_str = args
            .iter()
            .skip(1)
            .find(|arg| !arg.starts_with('-'))
            .ok_or_else(CliError::missing_repository)?;
        let repo = repo_str
            .parse()
            .map_err(error::CliError::invalid_repository)?;

        // Parse optional --fork flag
        let fork = args.iter().any(|arg| arg == "--fork");

        // Get HOME from environment
        let home = env::var("HOME")
            .map_err(|e| error::CliError::env_var_not_found(e))?
            .into();

        // Get GITHUB_TOKEN from environment
        let github_token =
            env::var("GITHUB_TOKEN").map_err(|e| error::CliError::env_var_not_found(e))?;

        Ok(Args {
            repo,
            fork,
            github_token,
            home,
        })
    }
}

#[cfg(test)]
mod tests {
    impl Args {
        fn test_new(repo_str: &str, fork: bool) -> Result<Self, error::CliError> {
            Ok(Self {
                repo: repo_str.parse()?,
                fork,
                github_token: "test-token".to_string(),
                home: PathBuf::from("/home/test"),
            })
        }
    }
    use super::*;

    #[test]
    fn test_parse_basic_args() {
        let args = Args::test_new("owner/repo", false).unwrap();
        assert_eq!(args.repo.owner, "owner");
        assert_eq!(args.repo.name, "repo");
        assert!(!args.fork);
        assert_eq!(args.github_token, "test-token");
        assert_eq!(args.home, PathBuf::from("/home/test"));
    }

    #[test]
    fn test_parse_with_fork_flag() {
        let args = Args::test_new("owner/repo", true).unwrap();
        assert!(args.fork);
        assert_eq!(args.repo.owner, "owner");
        assert_eq!(args.repo.name, "repo");
    }

    #[test]
    fn test_missing_repo() {
        assert!(Args::test_new("", false).is_err());
    }
}
