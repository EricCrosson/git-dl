use std::fmt::Display;

#[derive(Debug)]
#[non_exhaustive]
pub struct ParseError {
    input: String,
}

impl ParseError {
    pub(crate) fn invalid_format(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unrecognized repository format: {}", self.input)
    }
}

impl std::error::Error for ParseError {}
