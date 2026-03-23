use rusqlite::{Connection, Result};

use crate::Storage;
use nue_model::{CardID, NfcCard};

#[derive(Debug)]
pub struct SqliteStorage {
    connection: Connection,
}

impl SqliteStorage {
    pub fn new(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;
        Ok(Self { connection })
    }

    pub fn in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        Ok(Self { connection })
    }
}
