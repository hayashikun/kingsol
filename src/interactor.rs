use async_trait::async_trait;

use crate::repository::Repository;
use crate::use_case::*;

// GetLink
pub struct GetLink<R: Repository> {
    repository: R,
}

impl<R: Repository> GetLink<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: Repository> GetLinkUseCase for GetLink<R> {
    async fn handle(&self, input: GetLinkInput) -> Result<GetLinkOutput, AppError> {
        input.validate()?;
        let link = self.repository.get_link(input.key).await?;
        Ok(GetLinkOutput { link })
    }
}

// ListLinks
pub struct ListLinks<R: Repository> {
    repository: R,
}

impl<R: Repository> ListLinks<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: Repository> ListLinksUseCase for ListLinks<R> {
    async fn handle(&self, input: ListLinksInput) -> Result<ListLinksOutput, AppError> {
        input.validate()?;
        let links = self.repository.list_links().await?;
        Ok(ListLinksOutput { links })
    }
}

// CreateLink
pub struct CreateLink<R: Repository> {
    repository: R,
}

impl<R: Repository> CreateLink<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: Repository> CreateLinkUseCase for CreateLink<R> {
    async fn handle(&self, input: CreateLinkInput) -> Result<CreateLinkOutput, AppError> {
        input.validate()?;
        if input.overwrite {
            self.repository.upsert_link(input.link).await?;
        } else {
            self.repository.insert_link(input.link).await?;
        }
        Ok(CreateLinkOutput {})
    }
}
