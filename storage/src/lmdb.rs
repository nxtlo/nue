use heed::{Database, Env, EnvOpenOptions, WithTls, types};
use nue_model::{error::Error, raw_card::RawCard};

#[cfg(feature = "alloc")]
use alloc::collections::BTreeSet;

use core::borrow::Borrow;

use crate::Storage;

pub struct LmdbStorage {
    env: Env,
    db: Database<types::Bytes, types::Bytes>,
}

impl LmdbStorage {
    pub fn new(path: &'static str) -> Self {
        const MAP_SIZE: usize = 1024 * 1024 * 10; // 10 MB

        let env = unsafe {
            EnvOpenOptions::new()
                .max_dbs(1)
                .map_size(MAP_SIZE)
                .open(path)
                .expect("open env failure.")
        };
        let mut wtxn = env.write_txn().expect("write txn failure.");
        let db = env
            .create_database(&mut wtxn, Some(path))
            .expect("create database failure.");
        wtxn.commit().expect("commit failure.");
        Self { env: env, db }
    }

    #[inline]
    pub fn reader(&self) -> heed::RoTxn<'_, WithTls> {
        self.env.read_txn().expect("read txn err")
    }

    #[inline]
    pub fn writer(&self) -> heed::RwTxn<'_> {
        self.env.write_txn().expect("read txn err")
    }

    #[inline]
    pub fn database(&self) -> &Database<types::Bytes, types::Bytes> {
        &self.db
    }
}

impl Storage for LmdbStorage {
    type CardID = dyn Borrow<[u8]>;

    #[cfg(feature = "alloc")]
    type List = BTreeSet<NfcCard>;
    #[cfg(not(feature = "alloc"))]
    type List = ();

    fn get(&self, card_id: &Self::CardID) -> crate::Result<Option<RawCard>> {
        let reader = self.reader();
        if let Some(data) = self.db.get(&reader, card_id.borrow()).transpose() {
            Ok(RawCard::from_bytes(data.map_err(|_| Error::CardNotFound)?).copied())
        } else {
            Ok(None)
        }
    }

    fn put(&mut self, card_id: &Self::CardID, credential: RawCard) -> crate::Result<()> {
        let mut writer = self.writer();
        self.db
            .put(&mut writer, card_id.borrow(), credential.as_slice())
            .map_err(|_| Error::DBError)?;
        writer.commit().map_err(|_| Error::CommitError)?;
        Ok(())
    }

    fn update(&mut self, card_id: &Self::CardID, new: RawCard) -> crate::Result<()> {
        let mut writer = self.writer();
        self.db
            .put(&mut writer, card_id.borrow(), new.as_slice())
            .map_err(|_| Error::DBError)?;
        writer.commit().map_err(|_| Error::CommitError)?;
        Ok(())
    }

    fn delete(&mut self, card_id: &Self::CardID) -> crate::Result<()> {
        let mut writer = self.writer();
        self.db
            .delete(&mut writer, card_id.borrow())
            .map_err(|_| Error::DBError)?;
        writer.commit().map_err(|_| Error::CommitError)?;
        Ok(())
    }

    fn count(&self) -> crate::Result<usize> {
        let reader = self.reader();
        Ok(self.db.stat(&reader).map_err(|_| Error::DBError)?.entries)
    }

    #[cfg(not(feature = "alloc"))]
    fn list(&self) -> crate::Result<Self::List> {
        unimplemented!()
    }

    #[cfg(feature = "alloc")]
    fn list(&self) -> crate::Result<Self::List> {
        let reader = self.reader();
        let mut cards = Self::List::new();
        for item in self.db.iter(&reader)? {
            if let Ok((_, data)) = item {
                cards.insert(*NfcCard::from_bytes(data).ok_or_else(|| unreachable!())?);
            }
        }
        Ok(cards)
    }
}

#[cfg(test)]
impl Drop for LmdbStorage {
    fn drop(&mut self) {
        let mut writer = self.writer();
        let _ = self
            .db
            .clear(&mut writer)
            .and_then(move |()| writer.commit());
    }
}

#[cfg(test)]
mod tests {

    use nue_model::{auth::Token, raw_card::RawCard};

    use super::LmdbStorage;
    use crate::Result;
    use crate::Storage;

    #[test]
    fn test_put_and_get() -> Result<()> {
        let mut storage = LmdbStorage::new("/srv/db/test");
        let card = RawCard::new(Token::default());
        let uid = [0u8; 10];
        storage.put(&uid, card.clone())?;
        let result = storage.get(&uid).map(|r| r.unwrap())?;
        assert_eq!(card.as_slice(), result.as_slice());
        Ok(())
    }
}
