pub mod error;
pub mod github;
pub mod response_parser;

use self::error::GithubError;

pub trait FromResponse: Sized {
    fn from_response(response: &str) -> Result<Self, GithubError>;
}

pub trait GithubClient {
    fn get_repository(&self, repo: &crate::Repo) -> Result<GetRepositoryResponse, GithubError>;
    fn create_fork(&self, repo: &crate::Repo) -> Result<CreateForkResponse, GithubError>;
}

#[derive(Clone, Debug)]
pub struct GetRepositoryResponse {
    pub name: String,
    pub owner: GithubRepositoryOwner,
}

#[derive(Clone, Debug)]
pub struct GithubRepositoryOwner {
    pub login: String,
}

#[derive(Clone, Debug)]
pub struct CreateForkResponse {
    pub name: String,
    pub owner: GithubRepositoryOwner,
}
