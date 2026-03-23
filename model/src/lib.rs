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

pub mod auth;
pub mod card;
pub mod error;
pub mod utils;
