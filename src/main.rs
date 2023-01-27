#![forbid(unsafe_code)]
#![deny(warnings, missing_docs)]

//! Clone a git repository to a structured directory.

use std::{error::Error, path::PathBuf};

use clap::Parser;

mod error;
mod repo;

use crate::repo::{GetRepositoryResponse, Repo};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    /// Repository to clone
    repo: Repo,

    /// GitHub API token with `repo` permissions
    #[clap(long, env = "GITHUB_TOKEN")]
    github_token: String,

    /// Location of the user's home directory
    #[clap(long, env = "HOME")]
    home: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Cli {
        repo,
        github_token,
        home,
    } = Cli::parse();

    // Consult the GitHub API for the repository's proper capitalization
    let http_client = reqwest::blocking::Client::new();
    let response: GetRepositoryResponse = http_client
        .get(format!(
            "https://api.github.com/repos/{}/{}",
            repo.owner, repo.repository
        ))
        .header("User-Agent", "git-dl")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("token {}", github_token))
        .send()?
        .json()?;

    let repo: Repo = response.into();

    let directory = repo.directory(&home);
    if directory.exists() {
        eprintln!("Repository already cloned at {}", directory.display());
        return Ok(());
    }

    repo.clone(&home)
}
