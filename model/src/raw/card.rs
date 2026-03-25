use core::fmt;

use zerocopy::{AsBytes, FromBytes, FromZeroes};

use crate::auth::Token;
use crate::utils;

/// Represents an NFC card UID, Simply just an array of 10 bytes, with a few utility methods.
///
/// This struct implements [`zerocopy::AsBytes`], [`zerocopy::FromBytes`], and [`zerocopy::FromZeroes`]
/// so it can be turned into or from a byte slice.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, AsBytes, FromBytes, FromZeroes)]
#[repr(transparent)]
pub struct CardID(pub [u8; 10]);

impl From<[u8; 10]> for CardID {
    #[inline(always)]
    fn from(bytes: [u8; 10]) -> Self {
        Self(bytes)
    }
}

#[cfg(feature = "extras")]
impl serde::Serialize for CardID {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

#[cfg(feature = "extras")]
impl<'de> serde::Deserialize<'de> for CardID {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self(bytes))
    }
}

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

/// Represents data which physically lives inside an NFC card as bytes, which then can be read / written via [`nue_sys::App::read|write`].
///
/// Currently, this only stores an encrypted [`Token`] inside the card,
/// which is used to validate whether a card is allowed to perform certain actions, or not.
///
/// # Example
/// ```no_run
/// use nue_sys::{App, RawCard};
///
/// let mut ctx = App::context()?;
/// let mut app = App::uart(&mut ctx);
/// let mut incoming = app.incoming();
///
/// while let Some(Ok(raw_card)) = incoming.next() {
///     // ...process the card
/// }
/// ```
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
#[repr(transparent)] // This is currently transparent because it is only storing a token.
pub struct RawCard {
    auth: Token,
}

impl RawCard {
    #[inline]
    pub const fn new(auth: Token) -> Self {
        Self { auth }
    }

    /// Creates an [`RawCard`] initialized with zeros.
    #[inline]
    pub const fn zeroed() -> Self {
        Self {
            auth: Token {
                nonce: [0; 12],
                token: [0; 16],
                tag: [0; 16],
            },
        }
    }

    /// Returns this card as bytes, This can then be reconstructed from bytes using [`RawCard::from_bytes`].
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.as_bytes()
    }

    /// Attempts to reconstruct an [`RawCard`] from a byte slice.
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
