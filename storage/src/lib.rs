#![no_std]

/// Storage implementation for storing NFC cards.
///
/// This module provides storage implementations for storing NFC cards using different backends.
///
/// ## Avaliable Backends
///
/// - [`memory`]: In memory allocated BTreeMap storage implementation. `alloc` feature must be enabled.
/// - [`sqlite`]: SQLite storage implementation. `sqlite` feature must be enabled.
/// - [`lmdb`]: Lightning Memory-Mapped Database storage implementation.
/// Comes by default wth this crate. Specifically used to store and read RAW card data.
///
/// You can create your own storage implementation by implementing the [`Storage`] trait.

#[cfg(test)]
extern crate std;

#[cfg(any(feature = "alloc", feature = "sqlite"))]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// In memory allocated BTreeMap storage implementation.
#[cfg(feature = "alloc")]
pub mod memory;

/// SQLite storage implementation.
#[cfg(feature = "sqlite")]
pub mod sqlite;
#[cfg(feature = "sqlite")]
pub(crate) static SCHEMA: &str = include_str!("../../lib/schema.sql");
#[cfg(feature = "sqlite")]
use nue_model::card::NfcCard;

/// An LMDB storage implementation. works in no_std environments.
pub mod lmdb;
use core::borrow::Borrow;
use nue_model::{error::Result, raw_card::RawCard};

/// A trait for storage backends that can store `RawCard` NFC cards.
// TODO: Split this into Storage and RawStorage, corrosponding to NfcCard and RawCard respectively.
pub trait RawStorage {
    /// The type of the list of card IDs returned by [`list`].
    type List;
    /// The type of the card ID used by this storage.
    /// can be any type that can be borrowed as a slice of u8.
    type CardID: ?Sized + Borrow<[u8]>;

    fn get(&self, card_id: &Self::CardID) -> Result<Option<RawCard>>;
    fn put(&mut self, card_id: &Self::CardID, credential: RawCard) -> Result<()>;
    fn update(&mut self, card_id: &Self::CardID, new: RawCard) -> Result<()>;
    fn delete(&mut self, card_id: &Self::CardID) -> Result<()>;
    fn count(&self) -> Result<usize>;
    fn list(&self) -> Result<Self::List>;
}

#[cfg(any(feature = "alloc", feature = "sqlite"))]
pub trait Storage {
    fn get(&self, membership_id: usize) -> Result<NfcCard>;
    fn put(&mut self, membership_id: usize, credential: NfcCard) -> Result<()>;
    fn update(&mut self, membership_id: usize, new: NfcCard) -> Result<()>;
    fn delete(&mut self, membership_id: usize) -> Result<()>;
    fn count(&self) -> Result<usize>;
    fn list(&self) -> Result<alloc::vec::Vec<NfcCard>>;
}
