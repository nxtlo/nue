/// Impelemnts an authentication layer using ChaCha20Poly1305 for encryption.
use chacha20poly1305::{
    AeadInOut, ChaCha20Poly1305, Key, Nonce,
    aead::{KeyInit, Tag},
};

use crate::error::{Error, Result};

use zerocopy::{AsBytes, FromBytes, FromZeroes};

/// Represents a basic authentication token using ChaCha20Poly1305 encryption.
#[derive(
    // would be nice to have a type alias huh...
    AsBytes,
    FromBytes,
    FromZeroes,
    PartialEq,
    Eq,
    Default,
    Clone,
    Copy,
    Ord,
    PartialOrd,
)]
#[repr(C, packed)]
pub struct Token {
    pub(crate) nonce: [u8; 12],
    pub(crate) token: [u8; 16],
    pub(crate) tag: [u8; 16],
}

impl Token {
    #[inline]
    pub const fn from_raw(nonce: [u8; 12], token: [u8; 16], tag: [u8; 16]) -> Self {
        Self { nonce, token, tag }
    }

    #[inline]
    pub const fn token(&self) -> &[u8] {
        &self.token
    }

    #[inline]
    pub const fn nonce(&self) -> &[u8] {
        &self.nonce
    }

    #[inline]
    pub const fn tag(&self) -> &[u8] {
        &self.tag
    }
}

pub trait TokenStrategy {
    type Error: core::error::Error;

    /// Attempts to decrypt this token with the given key.
    ///
    /// # Example
    /// ```
    /// use nue_model::auth::Token;
    ///
    /// let very_secret_token: [u8; 32] = [0; 32];
    /// let (token, write_to_db) = Token::encrypt(&very_secret_token).unwrap();
    /// let decrypted = token.decrypt(&very_secret_token).unwrap();
    /// ```
    fn decrypt(&self, key: &[u8]) -> Result<[u8; 16]>;
}

pub trait TokenStrategyExt: TokenStrategy + Sized {
    /// Creates a new token encrypted with the given key, returning a tuple of (Self, [u8; 16])
    /// where the second value should be kept somewhere safe to later decrypt this the first value.
    ///
    /// Fails if the key is not 32 bytes long.
    ///
    /// # Example
    /// ```
    /// use nue_model::auth::Token;
    ///
    /// let very_secret_token: [u8; 32] = [0; 32];
    /// let (token, write_to_db) = Token::encrypt(&very_secret_token).unwrap();
    /// let decrypted = token.decrypt(&very_secret_token).unwrap();
    /// ```
    fn encrypt(key: &[u8]) -> Result<(Self, [u8; 16])>;
}

impl TokenStrategy for Token {
    type Error = chacha20poly1305::Error;

    fn decrypt(&self, key: &[u8]) -> Result<[u8; 16]> {
        let cipher = ChaCha20Poly1305::new(&Key::try_from(key).map_err(Error::from)?);
        let nonce = Nonce::from(self.nonce);
        let tag = Tag::<ChaCha20Poly1305>::from(self.tag);

        let mut token = self.token;
        cipher.decrypt_inout_detached(&nonce, b"", (&mut token[..]).into(), &tag)?;
        Ok(token)
    }
}

impl TokenStrategyExt for Token {
    fn encrypt(key: &[u8]) -> Result<(Self, [u8; 16])> {
        let mut nonce_bytes = [0u8; 12];
        let mut plaintext = [0u8; 16];

        getrandom::fill(&mut nonce_bytes).map_err(Error::from)?;
        getrandom::fill(&mut plaintext).map_err(Error::from)?;

        let cipher = ChaCha20Poly1305::new(&Key::try_from(key).map_err(Error::from)?);
        let nonce = Nonce::from(nonce_bytes);
        let mut ct = plaintext;
        let tag = cipher.encrypt_inout_detached(&nonce, b"", (&mut ct[..]).into())?;

        Ok((
            Self {
                nonce: nonce_bytes,
                token: ct,
                tag: tag.into(),
            },
            plaintext,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        let mut buf = [0; 32];
        getrandom::fill(&mut buf).unwrap();

        let (token, raw) = Token::encrypt(&buf).unwrap();
        let decrypted = token.decrypt(&buf).unwrap();

        assert_eq!(raw, decrypted);
    }
}
