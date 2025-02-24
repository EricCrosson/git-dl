use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::github_interface::{
    error::GithubError, CreateForkResponse, FromResponse, GetRepositoryResponse, GithubClient,
};

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
        let connector = TlsConnector::new().map_err(GithubError::request_failed)?;
        let stream =
            TcpStream::connect("api.github.com:443").map_err(GithubError::request_failed)?;
        let mut stream = connector
            .connect("api.github.com", stream)
            .map_err(GithubError::request_failed)?;

        let content_length = body.map(|b| b.len()).unwrap_or(0);
        let request = format!(
            "{} {} HTTP/1.1\r\n\
             Host: api.github.com\r\n\
             User-Agent: {}\r\n\
             Accept: application/vnd.github+json\r\n\
             Authorization: token {}\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            method,
            path,
            USER_AGENT,
            self.token,
            content_length,
            body.unwrap_or("")
        );

        stream
            .write_all(request.as_bytes())
            .map_err(GithubError::request_failed)?;

        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .map_err(GithubError::invalid_response)?;

        // Extract response body (after double CRLF)
        if let Some(idx) = response.find("\r\n\r\n") {
            Ok(response[idx + 4..].to_string())
        } else {
            Err(GithubError::invalid_response(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "malformed response",
            )))
        }
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
