use super::{
    CreateForkResponse, FromResponse, GetRepositoryResponse, GithubError, GithubRepositoryOwner,
};

/// Extract a string value from a JSON string at a given field path
fn extract_json_string<'a>(json: &'a str, field: &str) -> Result<&'a str, GithubError> {
    let mut pos = 0;
    let bytes = json.as_bytes();

    // Find the field
    while pos < bytes.len() {
        if let Some(idx) = &json[pos..].find(&format!("\"{}\"", field)) {
            pos += idx;
            // Skip past field name and colon
            pos += field.len() + 2;
            while pos < bytes.len() && bytes[pos] != b':' {
                pos += 1;
            }
            pos += 1;
            // Skip whitespace
            while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }
            // Expect a quote
            if pos >= bytes.len() || bytes[pos] != b'"' {
                return Err(GithubError::parse_error(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("expected string value for field {}", field),
                )));
            }
            pos += 1;
            let start = pos;
            // Find end of string
            while pos < bytes.len() && bytes[pos] != b'"' {
                if bytes[pos] == b'\\' {
                    pos += 2;
                } else {
                    pos += 1;
                }
            }
            if pos >= bytes.len() {
                return Err(GithubError::parse_error(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "unterminated string",
                )));
            }
            return Ok(&json[start..pos]);
        }
        pos += 1;
    }
    Err(GithubError::parse_error(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("field {} not found", field),
    )))
}

impl FromResponse for GetRepositoryResponse {
    fn from_response(response: &str) -> Result<Self, GithubError> {
        Ok(Self {
            name: extract_json_string(response, "name")?.to_string(),
            owner: GithubRepositoryOwner {
                login: extract_json_string(response, "login")?.to_string(),
            },
        })
    }
}

impl FromResponse for CreateForkResponse {
    fn from_response(response: &str) -> Result<Self, GithubError> {
        GetRepositoryResponse::from_response(response).map(|r| Self {
            name: r.name,
            owner: r.owner,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_string() {
        let input = r#"{"name": "test-repo", "owner": {"login": "test-user"}}"#;
        assert_eq!(extract_json_string(input, "name").unwrap(), "test-repo");
        assert_eq!(extract_json_string(input, "login").unwrap(), "test-user");
    }

    #[test]
    fn test_repository_response() {
        let input = r#"{"name": "test-repo", "owner": {"login": "test-user"}}"#;
        let response = GetRepositoryResponse::from_response(input).unwrap();
        assert_eq!(response.name, "test-repo");
        assert_eq!(response.owner.login, "test-user");
    }

    #[test]
    fn test_fork_response() {
        let input = r#"{"name": "test-repo", "owner": {"login": "test-user"}}"#;
        let response = CreateForkResponse::from_response(input).unwrap();
        assert_eq!(response.name, "test-repo");
        assert_eq!(response.owner.login, "test-user");
    }

    #[test]
    fn test_error_on_invalid_json() {
        let input = r#"{"name": "test-repo", "owner": {"login": }}"#;
        assert!(GetRepositoryResponse::from_response(input).is_err());
    }

    #[test]
    fn test_error_on_missing_fields() {
        let input = r#"{"name": "test-repo"}"#;
        assert!(GetRepositoryResponse::from_response(input).is_err());
    }
}
