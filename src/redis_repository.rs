use r2d2::{Error, Pool, PooledConnection};
use redis::{Client, Commands, RedisError};

use crate::entity::Link;
use crate::repository::{Repository, RepositoryError};

pub const LINK_KEY_PREFIX: &str = "link:";

struct RedisRepository {
    conn: PooledConnection<Client>,
}

impl From<Error> for RepositoryError {
    fn from(e: Error) -> Self {
        RepositoryError::Internal(e.to_string())
    }
}

impl From<RedisError> for RepositoryError {
    fn from(e: RedisError) -> Self {
        RepositoryError::Internal(e.to_string())
    }
}

impl RedisRepository {
    pub fn new(redis_pool: &Pool<Client>) -> Result<Self, RepositoryError> {
        let conn = redis_pool.get()
            .map_err(|e| RepositoryError::FailedToInitialize(e.to_string()))?;
        Ok(Self { conn })
    }
}

impl Repository for RedisRepository {
    fn get_link(&mut self, key: String) -> Result<Link, RepositoryError> {
        let uri: Option<String> = self.conn.get(format!("{}{}", LINK_KEY_PREFIX, key))?;
        if let Some(uri) = uri {
            Ok(Link { key, uri })
        } else {
            Err(RepositoryError::NotFound(format!("{} not found", key)))
        }
    }

    fn get_links(&mut self) -> Result<Vec<Link>, RepositoryError> {
        let keys_wp: Vec<String> = self.conn.keys(format!("{}*", LINK_KEY_PREFIX))?;
        let mut links = Vec::new();
        for k in keys_wp.iter() {
            let uri: Option<String> = self.conn.get(k)?;
            if let Some(uri) = uri {
                links.push(Link {
                    key: k.replace(LINK_KEY_PREFIX, ""),
                    uri,
                })
            }
        }
        Ok(links)
    }

    fn create_link(&mut self, link: Link) -> Result<(), RepositoryError> {
        if self.conn.exists(format!("{}{}", LINK_KEY_PREFIX, link.key))? {
            return Err(RepositoryError::AlreadyExists(format!("{} already exists", link.key)));
        }
        self.conn.set(format!("{}{}", LINK_KEY_PREFIX, link.key), link.uri)?;
        Ok(())
    }
}
