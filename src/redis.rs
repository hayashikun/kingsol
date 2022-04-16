extern crate redis;

use anyhow::{Context, Result};
use r2d2::{Pool, PooledConnection};
use redis::Client;

pub fn create_connection_pool(host: &str) -> Result<Pool<Client>> {
    let client = Client::open(host)?;
    r2d2::Pool::builder().build(client).context("Failed to create connection pool")
}

pub fn get_connection(pool: &r2d2::Pool<Client>) -> Result<PooledConnection<Client>> {
    pool.get().context("Failed to get connection")
}

pub const LINK_KEY_PREFIX: &str = "link";
