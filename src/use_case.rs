use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};

use async_trait::async_trait;

use crate::entity::Link;
use crate::repository::RepositoryError;

#[derive(Debug, PartialEq)]
pub enum AppError {
    ValidationError(String),
    NotFound(String),
    AlreadyExists(String),
    InvalidArgument(String),
    Internal(String),
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            AppError::ValidationError(s) => write!(f, "ValidationError {}", s),
            AppError::NotFound(s) => write!(f, "NotFound {}", s),
            AppError::AlreadyExists(s) => write!(f, "AlreadyExists {}", s),
            AppError::InvalidArgument(s) => write!(f, "InvalidArgument {}", s),
            AppError::Internal(s) => write!(f, "Internal {}", s),
        }
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

#[async_trait]
pub trait GetLinkUseCase {
    async fn handle(&self, input: GetLinkInput) -> Result<GetLinkOutput, AppError>;
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

#[async_trait]
pub trait ListLinksUseCase {
    async fn handle(&self, input: ListLinksInput) -> Result<ListLinksOutput, AppError>;
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

#[async_trait]
pub trait CreateLinkUseCase {
    async fn handle(&self, input: CreateLinkInput) -> Result<CreateLinkOutput, AppError>;
}
