use crate::repository::Repository;
use crate::use_case::{*};

struct GetLink {
    repository: dyn Repository,
}

impl GetLinkUseCase for GetLink {
    fn handle(&mut self, input: GetLinkInput) -> Result<GetLinkOutput, AppError> {
        let link = self.repository.get_link(input.key)?;
        Ok(GetLinkOutput { link })
    }
}

struct ListLinks {
    repository: dyn Repository,
}

impl ListLinksUseCase for ListLinks {
    fn handle(&mut self, _input: ListLinksInput) -> Result<ListLinksOutput, AppError> {
        let links = self.repository.list_links()?;
        Ok(ListLinksOutput { links })
    }
}

struct CreateLink {
    repository: dyn Repository,
}

impl CreateLinkUseCase for CreateLink {
    fn handle(&mut self, input: CreateLinkInput) -> Result<CreateLinkOutput, AppError> {
        self.repository.create_link(input.link)?;
        Ok(CreateLinkOutput {})
    }
}
