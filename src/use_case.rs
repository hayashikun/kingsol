use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};

use crate::entity::Link;

#[derive(Debug, PartialEq)]
pub enum AppError {
    RepositoryError(String),
    ValidationError(String),
    NotFound(String),
    AlreadyExists(String),
    InvalidArgument(String),
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", self)
    }
}

struct GetLinkInput {
    pub key: String,
}

struct GetLinkOutput {
    pub link: Link,
}

trait GetLinkUseCase {
    fn handle(request: GetLinkInput) -> Result<GetLinkOutput, AppError>;
}

struct ListLinksInput {}

struct ListLinksOutput {
    pub links: Vec<Link>,
}

trait ListLinksUseCase {
    fn handle(request: ListLinksInput) -> Result<ListLinksOutput, AppError>;
}

struct CreateLinkInput {}

struct CreateLinkOutput {
    pub links: Vec<Link>,
}

trait CreateLinkUseCase {
    fn handle(request: CreateLinkInput) -> Result<CreateLinkOutput, AppError>;
}
