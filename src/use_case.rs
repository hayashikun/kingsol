use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};

use crate::entity::Link;
use crate::repository::RepositoryError;

#[derive(Debug, PartialEq)]
pub enum AppError {
    RepositoryError(String),
    ValidationError(String),
    NotFound(String),
    AlreadyExists(String),
    InvalidArgument(String),
    Internal(String),
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", self)
    }
}

impl From<RepositoryError> for AppError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::NotFound(s) => AppError::NotFound(s),
            RepositoryError::AlreadyExists(s) => AppError::AlreadyExists(s),
            RepositoryError::InvalidArgument(s) => AppError::InvalidArgument(s),
            _ => AppError::Internal(e.to_string()),
        }
    }
}


pub struct GetLinkInput {
    pub key: String,
}

pub struct GetLinkOutput {
    pub link: Link,
}

pub trait GetLinkUseCase {
    fn handle(&mut self, input: GetLinkInput) -> Result<GetLinkOutput, AppError>;
}

pub struct ListLinksInput {}

pub struct ListLinksOutput {
    pub links: Vec<Link>,
}

pub trait ListLinksUseCase {
    fn handle(&mut self, input: ListLinksInput) -> Result<ListLinksOutput, AppError>;
}

pub struct CreateLinkInput {
    pub link: Link,
}

pub struct CreateLinkOutput {}

pub trait CreateLinkUseCase {
    fn handle(&mut self, input: CreateLinkInput) -> Result<CreateLinkOutput, AppError>;
}
