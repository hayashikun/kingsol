use crate::repository::Repository;
use crate::use_case::{*};

// GetLink
pub struct GetLink<R: Repository> {
    repository: R,
}

impl<R: Repository> GetLink<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: Repository> GetLinkUseCase for GetLink<R> {
    fn handle(&mut self, input: GetLinkInput) -> Result<GetLinkOutput, AppError> {
        input.validate()?;
        let link = self.repository.get_link(input.key)?;
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

impl<R: Repository> ListLinksUseCase for ListLinks<R> {
    fn handle(&mut self, input: ListLinksInput) -> Result<ListLinksOutput, AppError> {
        input.validate()?;
        let links = self.repository.list_links()?;
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

impl<R: Repository> CreateLinkUseCase for CreateLink<R> {
    fn handle(&mut self, input: CreateLinkInput) -> Result<CreateLinkOutput, AppError> {
        input.validate()?;
        if input.overwrite {
            self.repository.upsert_link(input.link)?;
        } else {
            self.repository.insert_link(input.link)?;
        }
        Ok(CreateLinkOutput {})
    }
}
