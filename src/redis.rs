extern crate redis;

use anyhow::{Context, Result};
use r2d2::Pool;
use redis::{Client, IntoConnectionInfo};

pub fn create_connection_pool<T: IntoConnectionInfo>(host: T) -> Result<Pool<Client>> {
    let client = Client::open(host).context("Failed to open host")?;
    Pool::builder()
        .build(client)
        .context("Failed to build pool")
}
