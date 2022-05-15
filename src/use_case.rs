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

impl GetLinkInput {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.key == "" {
            return Err(AppError::ValidationError("empty key".to_string()));
        }
        Ok(())
    }
}

pub struct GetLinkOutput {
    pub link: Link,
}

pub trait GetLinkUseCase {
    fn handle(&mut self, input: GetLinkInput) -> Result<GetLinkOutput, AppError>;
}

pub struct ListLinksInput {}

impl ListLinksInput {
    pub fn validate(&self) -> Result<(), AppError> {
        Ok(())
    }
}

pub struct ListLinksOutput {
    pub links: Vec<Link>,
}

pub trait ListLinksUseCase {
    fn handle(&mut self, input: ListLinksInput) -> Result<ListLinksOutput, AppError>;
}

pub struct CreateLinkInput {
    pub overwrite: bool,
    pub link: Link,
}

impl CreateLinkInput {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.link.key == "" {
            return Err(AppError::ValidationError("empty key".to_string()));
        }
        if self.link.uri == "" {
            return Err(AppError::ValidationError("empty uri".to_string()));
        }
        Ok(())
    }
}

pub struct CreateLinkOutput {}

pub trait CreateLinkUseCase {
    fn handle(&mut self, input: CreateLinkInput) -> Result<CreateLinkOutput, AppError>;
}
