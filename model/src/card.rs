use serde::{Deserialize, Serialize};
use zerocopy::FromZeroes;

use crate::raw::card::CardID;
use alloc::sync::Arc;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum SubTier {
    #[default]
    Basic,
    VIP,
}

impl From<u8> for SubTier {
    fn from(value: u8) -> Self {
        match value {
            0 => SubTier::Basic,
            _ => SubTier::VIP,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum SubStatus {
    Active,
    #[default]
    Inactive,
}

impl From<u8> for SubStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => SubStatus::Active,
            _ => SubStatus::Inactive,
        }
    }
}

/// The result of reading an NFC card from a database call.
///
/// Thie differs from [`RawCard`] in that it includes additional fields fetched from an external resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfcCard {
    username: Arc<str>,
    subscription_start: chrono::DateTime<chrono::Utc>,
    subscription_expiry: chrono::DateTime<chrono::Utc>,
    last_used: chrono::DateTime<chrono::Utc>,
    uid: CardID,
    tier: SubTier,
    status: SubStatus,
}

impl NfcCard {
    pub fn new(
        username: impl Into<Arc<str>>,
        subscription_start: chrono::DateTime<chrono::Utc>,
        subscription_expiry: chrono::DateTime<chrono::Utc>,
        last_used: chrono::DateTime<chrono::Utc>,
        uid: CardID,
        tier: SubTier,
        status: SubStatus,
    ) -> Self {
        Self {
            username: username.into(),
            subscription_start,
            subscription_expiry,
            last_used,
            uid,
            tier,
            status,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub const fn subscription_start(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.subscription_start
    }

    pub const fn subscription_expiry(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.subscription_expiry
    }

    pub const fn last_used(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.last_used
    }

    pub const fn uid(&self) -> CardID {
        self.uid
    }

    pub const fn tier(&self) -> SubTier {
        self.tier
    }

    pub const fn status(&self) -> SubStatus {
        self.status
    }
}

pub struct NfcCardBuilder {
    username: Option<Arc<str>>,
    subscription_start: Option<chrono::DateTime<chrono::Utc>>,
    subscription_expiry: Option<chrono::DateTime<chrono::Utc>>,
    last_used: Option<chrono::DateTime<chrono::Utc>>,
    uid: Option<CardID>,
    tier: Option<SubTier>,
    status: Option<SubStatus>,
}

impl NfcCardBuilder {
    pub const fn new() -> Self {
        Self {
            username: None,
            subscription_start: None,
            subscription_expiry: None,
            last_used: None,
            uid: None,
            tier: None,
            status: None,
        }
    }

    pub fn username(mut self, username: impl Into<Arc<str>>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub const fn start(mut self, start: isize) -> Self {
        self.subscription_start = Some(
            chrono::DateTime::from_timestamp(start as i64, 0).expect("invalid start timestamp."),
        );
        self
    }

    pub const fn expiry(mut self, expiry: isize) -> Self {
        self.subscription_expiry = Some(
            chrono::DateTime::from_timestamp(expiry as i64, 0).expect("invalid expiry timestamp."),
        );
        self
    }

    pub const fn last_used(mut self, last_used: isize) -> Self {
        self.last_used = Some(
            chrono::DateTime::from_timestamp(last_used as i64, 0)
                .expect("invalid last used timestamp."),
        );
        self
    }

    pub const fn uid(mut self, uid: CardID) -> Self {
        self.uid = Some(uid);
        self
    }

    pub const fn tier(mut self, tier: SubTier) -> Self {
        self.tier = Some(tier);
        self
    }

    pub const fn status(mut self, status: SubStatus) -> Self {
        self.status = Some(status);
        self
    }

    #[inline]
    pub fn finish(self) -> NfcCard {
        NfcCard {
            username: self.username.unwrap_or_default(),
            subscription_start: self.subscription_start.unwrap_or_default(),
            subscription_expiry: self.subscription_expiry.unwrap_or_default(),
            last_used: self.last_used.unwrap_or_default(),
            uid: self.uid.unwrap_or_else(CardID::new_zeroed),
            tier: self.tier.unwrap_or_default(),
            status: self.status.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    let card = NfcCardBuilder::new()
        .username("test")
        .uid(CardID::new_zeroed())
        .finish();
    assert!(card.username() == "test" && card.uid().as_slice() == [0; 10]);
}
