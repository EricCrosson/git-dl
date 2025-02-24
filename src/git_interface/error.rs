use std::fmt::Display;

#[derive(Debug)]
#[non_exhaustive]
pub struct GitError {
    kind: GitErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

#[derive(Debug)]
pub enum GitErrorKind {
    CloneFailed,
    RemoteAddFailed,
}

impl GitError {
    pub(crate) fn clone_failed(source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            kind: GitErrorKind::CloneFailed,
            source: Some(Box::new(source)),
        }
    }

    pub(crate) fn remote_add_failed(
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind: GitErrorKind::RemoteAddFailed,
            source: Some(Box::new(source)),
        }
    }
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            GitErrorKind::CloneFailed => write!(f, "failed to clone git repository"),
            GitErrorKind::RemoteAddFailed => write!(f, "failed to add git remote"),
        }
    }
}

impl std::error::Error for GitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as _)
    }
}
