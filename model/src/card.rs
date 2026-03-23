use core::fmt;

use zerocopy::{AsBytes, FromBytes, FromZeroes};

use crate::auth;
use crate::utils;

/// Represents an NFC card ID.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, AsBytes, FromBytes, FromZeroes)]
#[repr(transparent)]
pub struct CardID(pub [u8; 10]);

impl CardID {
    /// Get a view of the underlying card ID bytes as a string slice.
    #[inline]
    pub const fn as_str(&self) -> &str {
        static mut PARSER: &mut [u8; 11] = &mut [0; 11];
        // SAFETY:
        // 1: Buffer size fits `buf`.
        // 2: PARSER is only used in single thread context.
        utils::parse_uid(&self.0, unsafe { PARSER })
    }

    /// Get a view of the underlying card array as a slice.
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

/// Represents data which exists inside an NFC card. currently, this is only an encrypted token.
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
#[repr(C, packed)]
pub struct NfcCard {
    auth: auth::Token,
}

impl NfcCard {
    #[inline]
    pub const fn new(auth: auth::Token) -> Self {
        Self { auth }
    }

    /// Creates an [`NfcCard`] initialized with zeros.
    #[inline]
    pub const fn zeroed() -> Self {
        Self {
            auth: auth::Token {
                nonce: [0; 12],
                token: [0; 16],
                tag: [0; 16],
            },
        }
    }

    /// Returns this card as bytes, This can then be reconstructed from bytes using [`NfcCard::from_bytes`].
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.as_bytes()
    }

    /// Attempts to reconstruct an [`NfcCard`] from a byte slice.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        Self::ref_from(bytes)
    }
}

impl AsRef<[u8]> for CardID {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl fmt::Display for CardID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.pad(self.as_str())
    }
}
