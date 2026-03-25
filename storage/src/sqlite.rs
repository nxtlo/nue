extern crate alloc;

use alloc::string::String;
use rusqlite::{Connection, Result};

use alloc::vec::Vec;
use core::borrow::Borrow;
use nue_model::{card::*, raw_card::CardID};

#[derive(Debug)]
pub struct SqliteStorage {
    connection: Connection,
}

impl SqliteStorage {
    pub fn into_inner(self) -> Connection {
        self.connection
    }

    pub fn open(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;
        Ok(Self { connection })
    }

    pub fn in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        Ok(Self { connection })
    }

    pub fn create_tables(&self) -> Result<()> {
        self.connection.execute_batch(crate::SCHEMA)?;
        Ok(())
    }
}
