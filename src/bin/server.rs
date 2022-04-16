use anyhow::Result;

use kingsol::server;

pub fn main() -> Result<()> {
    server::start()
}
