#![forbid(unsafe_code)]
// TODO:
// #![deny(warnings, missing_docs)]

//! Clone a git repository to a structured directory.

use std::{error::Error, path::PathBuf};

use clap::Parser;

mod error;
mod repo;

use crate::repo::Repo;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Repository to clone
    repo: Repo,

    #[clap(long, env = "HOME")]
    home: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Cli { repo, home } = Cli::parse();

    let directory = repo.directory(&home);
    if directory.exists() {
        eprintln!("Repository already cloned at {}", directory.display());
        return Ok(());
    }

    repo.clone(&home)
}
