use crate::repository::Repository;
use crate::use_case::*;

// GetLink
pub struct GetLink<'a, R: Repository> {
    repository: &'a mut R,
}

impl<'a, R: Repository> GetLink<'a, R> {
    pub fn new(repository: &'a mut R) -> Self {
        Self { repository }
    }
}

impl<'a, R: Repository> GetLinkUseCase for GetLink<'a, R> {
    fn handle(&mut self, input: GetLinkInput) -> Result<GetLinkOutput, AppError> {
        input.validate()?;
        let link = self.repository.get_link(input.key)?;
        Ok(GetLinkOutput { link })
    }
}

// ListLinks
pub struct ListLinks<'a, R: Repository> {
    repository: &'a mut R,
}

impl<'a, R: Repository> ListLinks<'a, R> {
    pub fn new(repository: &'a mut R) -> Self {
        Self { repository }
    }
}

impl<'a, R: Repository> ListLinksUseCase for ListLinks<'a, R> {
    fn handle(&mut self, input: ListLinksInput) -> Result<ListLinksOutput, AppError> {
        input.validate()?;
        let links = self.repository.list_links()?;
        Ok(ListLinksOutput { links })
    }
}

// CreateLink
pub struct CreateLink<'a, R: Repository> {
    repository: &'a mut R,
}

impl<'a, R: Repository> CreateLink<'a, R> {
    pub fn new(repository: &'a mut R) -> Self {
        Self { repository }
    }
}

impl<'a, R: Repository> CreateLinkUseCase for CreateLink<'a, R> {
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
