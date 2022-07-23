use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};

use async_trait::async_trait;

use crate::entity::{Link, Token};

#[derive(Debug, PartialEq)]
pub enum RepositoryError {
    NotFound(String),
    AlreadyExists(String),
    InvalidArgument(String),
    Internal(String),
}

impl Error for RepositoryError {}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            RepositoryError::NotFound(s) => write!(f, "NotFound {}", s),
            RepositoryError::AlreadyExists(s) => write!(f, "AlreadyExists {}", s),
            RepositoryError::InvalidArgument(s) => write!(f, "InvalidArgument {}", s),
            RepositoryError::Internal(s) => write!(f, "Internal {}", s),
        }
    }
}

#[async_trait]
pub trait Repository: Sync + Send {
    async fn get_link(&self, key: String) -> Result<Link, RepositoryError>;
    async fn list_links(&self) -> Result<Vec<Link>, RepositoryError>;
    async fn insert_link(&self, link: Link) -> Result<(), RepositoryError>;
    async fn upsert_link(&self, link: Link) -> Result<(), RepositoryError>;
    async fn exist_token(&self, token: Token) -> Result<bool, RepositoryError>;
}
