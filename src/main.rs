#![forbid(unsafe_code)]
#![deny(warnings, missing_docs)]

//! Clone a git repository to a structured directory.

mod cli;
mod git_interface;
mod github_interface;
mod little_anyhow;
mod repo;

use cli::Args;
use git_interface::{git::SystemGit, GitOperations};
use github_interface::{github::NativeGithubClient, GithubClient};
use repo::Repo;

fn main() -> Result<(), little_anyhow::Error> {
    let Args {
        repo,
        fork,
        github_token,
        home,
    } = Args::parse()?;

    let github = NativeGithubClient::new(github_token);
    let git = SystemGit::new();

    // Get repository info from GitHub API
    let repo_info = github.get_repository(&repo)?;
    let repo: Repo = repo_info.into();

    let directory = repo.directory(&home);
    if directory.exists() {
        eprintln!("Repository already cloned at {}", directory.display());
        return Ok(());
    }

    if !fork {
        return Ok(git.clone(&repo.to_string(), &directory)?);
    }

    // Create the fork
    let fork_info = github.create_fork(&repo)?;

    // Clone the original repository
    git.clone(&repo.to_string(), &directory)?;

    // Add the fork as a remote
    let fork_url = format!(
        "git@github.com:{}/{}.git",
        fork_info.owner.login, fork_info.name
    );
    git.add_remote(&directory, "fork", &fork_url)?;

    Ok(())
}
