#![forbid(unsafe_code)]
#![deny(warnings, missing_docs)]

//! Clone a git repository to a structured directory.

use std::{error::Error, path::PathBuf, process::Command};

use clap::Parser;
use repo::GithubRepositoryOwner;
use serde::{Deserialize, Serialize};

mod error;
mod repo;

use crate::repo::{GetRepositoryResponse, Repo};

const USER_AGENT: &'static str = "EricCrosson/git-dl";

#[derive(Clone, Debug, Serialize)]
struct CreateForkRequest {
    name: String,
    default_branch_only: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct CreateForkResponse {
    name: String,
    owner: GithubRepositoryOwner,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    /// Repository to clone
    repo: Repo,

    /// Create a fork of the target repository
    #[clap(long)]
    pub fork: bool,

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
        fork,
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
        .header("User-Agent", USER_AGENT)
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

    if fork {
        // Create the fork (which GitHub handles asynchronously)
        let response: CreateForkResponse = http_client
            .post(format!(
                "https://api.github.com/repos/{}/{}/forks",
                repo.owner, repo.repository
            ))
            .header("User-Agent", USER_AGENT)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("token {}", github_token))
            .json(&CreateForkRequest {
                name: repo.repository.clone(),
                default_branch_only: true,
            })
            .send()?
            .json()?;

        // Add the remote to the cloned repository
        Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("fork")
            .arg(format!(
                "git@github.com:{}/{}.git",
                response.owner.login, response.name
            ));
    }

    repo.clone(&home)
}
