extern crate alloc;

use alloc::vec::Vec;

use rusqlite::{Connection, Result};

use nue_model::{Error, Result as NueResult, card::NfcCard};

use crate::Storage;

#[derive(Debug)]
pub struct SqliteStorage {
    connection: Connection,
}

impl SqliteStorage {
    pub fn into_inner(self) -> Connection {
        self.connection
    }

    pub fn open<P: AsRef<str> + ?Sized>(path: &P) -> Result<Self> {
        let connection = Connection::open(path.as_ref())?;
        let slf = Self { connection };
        slf.create_tables()?;
        Ok(slf)
    }

    pub fn in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        let slf = Self { connection };
        slf.create_tables()?;
        Ok(slf)
    }

    pub fn create_tables(&self) -> Result<()> {
        self.connection.execute_batch(crate::SCHEMA)?;
        Ok(())
    }
}

impl Storage for SqliteStorage {
    fn get(&self, membership_id: usize) -> NueResult<NfcCard> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM NfcCard WHERE membership_id = ?")
            .map_err(Error::from)?;

        stmt.query_row((membership_id as i64,), |row| NfcCard::try_from(row))
            .map_err(Error::from)
    }

    fn put(&mut self, membership_id: usize, credential: NfcCard) -> NueResult<()> {
        let mut stmt = self
            .connection
            .prepare(r#"
                INSERT OR IGNORE INTO NfcCard (uid, username, membership_id, subscription_status, subscription_tier, subscription_start, subscription_end, last_used)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?);
            "#)
            .map_err(Error::from)?;
        stmt.execute((
            credential.uid().as_slice(),
            credential.username(),
            membership_id as i64,
            credential.subscription_status() as u8,
            credential.subscription_tier() as u8,
            credential.subscription_start().map(|ts| ts.timestamp()),
            credential.subscription_end().map(|ts| ts.timestamp()),
            credential.last_used().map(|ts| ts.timestamp()),
        ))
        .map_err(Error::from)?;
        Ok(())
    }

    fn update(&mut self, membership_id: usize, new: NfcCard) -> NueResult<()> {
        let mut stmt = self
            .connection
            .prepare(r#"
                UPDATE NfcCard
                SET username = ?, subscription_status = ?, subscription_tier = ?, subscription_start = ?, subscription_end = ?, last_used = ?
                WHERE membership_id = ?;
            "#)
            .map_err(Error::from)?;
        stmt.execute((
            new.username(),
            new.subscription_status() as u8,
            new.subscription_tier() as u8,
            new.subscription_start().map(|ts| ts.timestamp()),
            new.subscription_end().map(|ts| ts.timestamp()),
            new.last_used().map(|ts| ts.timestamp()),
            membership_id as i64,
        ))
        .map_err(Error::from)?;
        Ok(())
    }

    fn delete(&mut self, membership_id: usize) -> NueResult<()> {
        let mut stmt = self
            .connection
            .prepare("DELETE FROM NfcCard WHERE membership_id = ?")
            .map_err(Error::from)?;
        stmt.execute((membership_id as i64,)).map_err(Error::from)?;
        Ok(())
    }

    fn count(&self) -> NueResult<usize> {
        let mut stmt = self
            .connection
            .prepare("SELECT COUNT(membership_id) FROM NfcCard")
            .map_err(Error::from)?;
        let count: usize = stmt.query_row((), |row| row.get::<_, i64>(0).map(|v| v as usize))?;
        Ok(count)
    }

    fn list(&self) -> NueResult<Vec<NfcCard>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM NfcCard")
            .map_err(Error::from)?;

        let mut rows = stmt.query_map((), |row| NfcCard::try_from(row))?;
        let mut v = Vec::with_capacity(rows.size_hint().0);

        while let Some(row) = rows.next() {
            v.push(row?)
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::{SqliteStorage, Storage};
    use nue_model::card::NfcCardBuilder;

    #[test]
    fn test_in_memory() {
        let mut db = SqliteStorage::in_memory().unwrap();
        db.put(
            1,
            NfcCardBuilder::new()
                .uid([0; 10].into())
                .username("faris")
                .membership_id(1)
                .subscription_status(1.into())
                .subscription_tier(1.into())
                .finish(),
        )
        .unwrap();

        assert!(db.count().is_ok_and(|c| c == 1))
    }
}
