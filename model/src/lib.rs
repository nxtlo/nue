#![no_std]

/// Core data models and types shared across nue crates, Works well in `no_std` environments.
///
/// This exports:
/// - error::{Error, Result}, for error propagation
/// - auth::Token, for basic authentication tokens.
/// -
extern crate alloc;

#[cfg(test)]
extern crate std;

pub mod error;
pub mod raw;
pub(crate) mod utils;

pub use error::{Error, Result};
pub use raw::{auth, card as raw_card};

#[cfg(feature = "extras")]
pub mod card;
