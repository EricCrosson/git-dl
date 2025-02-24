use crate::github_interface::{
    error::GithubError, CreateForkResponse, FromResponse, GetRepositoryResponse, GithubClient,
};
use std::process::Command;

const USER_AGENT: &str = "EricCrosson/git-dl";

pub struct NativeGithubClient {
    token: String,
}

impl NativeGithubClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    fn make_request(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<String, GithubError> {
        let url = format!("https://api.github.com{}", path);
        let mut cmd = Command::new("curl");

        cmd.arg("--silent")
            .arg("--show-error")
            .arg("--request")
            .arg(method)
            .arg("--header")
            .arg(format!("User-Agent: {}", USER_AGENT))
            .arg("--header")
            .arg("Accept: application/vnd.github+json")
            .arg("--header")
            .arg(format!("Authorization: token {}", self.token));

        if let Some(body_content) = body {
            cmd.arg("--header")
                .arg("Content-Type: application/json")
                .arg("--data")
                .arg(body_content);
        }

        cmd.arg(url);

        let output = cmd.output().map_err(GithubError::request_failed)?;

        if !output.status.success() {
            return Err(GithubError::request_failed(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            )));
        }

        String::from_utf8(output.stdout).map_err(|e| GithubError::invalid_response(e))
    }
}

impl GithubClient for NativeGithubClient {
    fn get_repository(&self, repo: &crate::Repo) -> Result<GetRepositoryResponse, GithubError> {
        let path = format!("/repos/{}/{}", repo.owner, repo.name);
        let response = self.make_request("GET", &path, None)?;
        GetRepositoryResponse::from_response(&response)
    }

    fn create_fork(&self, repo: &crate::Repo) -> Result<CreateForkResponse, GithubError> {
        let path = format!("/repos/{}/{}/forks", repo.owner, repo.name);
        let body = format!(r#"{{"name":"{}","default_branch_only":true}}"#, repo.name);
        let response = self.make_request("POST", &path, Some(&body))?;
        CreateForkResponse::from_response(&response)
    }
}
