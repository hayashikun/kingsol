use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};

use crate::entity::Link;

#[derive(Debug, PartialEq)]
pub enum RepositoryError {
    NotFound(String),
    AlreadyExists(String),
    InvalidArgument(String),
    FailedToInitialize(String),
    Internal(String),
}

impl Error for RepositoryError {}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", self)
    }
}

pub trait Repository {
    fn get_link(&mut self, key: String) -> Result<Link, RepositoryError>;
    fn list_links(&mut self) -> Result<Vec<Link>, RepositoryError>;
    fn insert_link(&mut self, link: Link) -> Result<(), RepositoryError>;
    fn upsert_link(&mut self, link: Link) -> Result<(), RepositoryError>;
    fn exist_token(&mut self, link: Link) -> Result<(), RepositoryError>;
}
