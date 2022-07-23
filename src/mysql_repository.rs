use async_trait::async_trait;
use sqlx::{Error, MySql, Pool};

use crate::entity::{Link, Token};
use crate::repository::{Repository, RepositoryError};

pub struct MysqlRepository<'a> {
    pool: &'a Pool<MySql>,
}

#[derive(sqlx::FromRow)]
struct LinkRecord {
    key: String,
    uri: String,
}

impl From<LinkRecord> for Link {
    fn from(r: LinkRecord) -> Self {
        Link {
            key: r.key,
            uri: r.uri,
        }
    }
}

impl From<Error> for RepositoryError {
    fn from(e: Error) -> Self {
        match e {
            Error::RowNotFound => RepositoryError::NotFound("not found".to_string()),
            _ => RepositoryError::Internal(e.to_string()),
        }
    }
}

impl<'a> MysqlRepository<'a> {
    pub fn new(pool: &'a Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'a> Repository for MysqlRepository<'a> {
    async fn get_link(&self, key: String) -> Result<Link, RepositoryError> {
        let record: LinkRecord = sqlx::query_as("SELECT `key`, `uri` FROM `links` WHERE `key` = ?")
            .bind(key)
            .fetch_one(self.pool)
            .await?;
        Ok(Link::from(record))
    }

    async fn list_links(&self) -> Result<Vec<Link>, RepositoryError> {
        let records = sqlx::query_as::<_, LinkRecord>("SELECT `key`, `uri` FROM `links`")
            .fetch_all(self.pool)
            .await?;
        Ok(records.into_iter().map(Link::from).collect())
    }

    async fn insert_link(&self, link: Link) -> Result<(), RepositoryError> {
        match self.get_link(link.key.clone()).await {
            Ok(_) => Err(RepositoryError::AlreadyExists(link.key.clone())),
            Err(RepositoryError::NotFound(_s)) => Ok(()),
            Err(e) => Err(e),
        }?;

        let _ = sqlx::query("INSERT INTO `links` (`key`, `uri`) VALUES (?, ?)")
            .bind(&link.key)
            .bind(&link.uri)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    async fn upsert_link(&self, link: Link) -> Result<(), RepositoryError> {
        let q = match self.get_link(link.key.clone()).await {
            Ok(_) => {
                let q = sqlx::query("UPDATE `links` SET `uri` = ? WHERE `key` = ?")
                    .bind(&link.uri)
                    .bind(&link.key);
                Ok(q)
            }
            Err(RepositoryError::NotFound(_s)) => {
                let q = sqlx::query("INSERT INTO `links` (`key`, `uri`) VALUES (?, ?)")
                    .bind(&link.key)
                    .bind(&link.uri);
                Ok(q)
            }
            Err(e) => Err(e),
        }?;
        let _ = q.execute(self.pool).await?;
        Ok(())
    }

    async fn exist_token(&self, token: Token) -> Result<bool, RepositoryError> {
        let result =
            sqlx::query_as::<_, (String,)>("SELECT `token` FROM `tokens` WHERE `token` = ?")
                .bind(token)
                .fetch_one(self.pool)
                .await;
        match result {
            Ok(_) => Ok(true),
            Err(Error::RowNotFound) => Ok(false),
            Err(e) => Err(RepositoryError::from(e)),
        }
    }
}
