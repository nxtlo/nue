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
    uid: CardID,
    username: Arc<str>,
    membership_id: usize,
    subscription_status: SubStatus,
    subscription_tier: SubTier,
    subscription_start: Option<chrono::DateTime<chrono::Utc>>,
    subscription_end: Option<chrono::DateTime<chrono::Utc>>,
    last_used: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<&rusqlite::Row<'_>> for NfcCard {
    type Error = rusqlite::Error;

    /// Attempts to convert a [`rusqlite::Row`] into an [`NfcCard`].
    fn try_from(row: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            username: row.get("username")?,
            uid: row.get::<_, [u8; 10]>("uid").map(Into::into)?,
            membership_id: row.get::<_, i64>("membership_id")? as usize,
            subscription_tier: row.get::<_, u8>("subscription_tier").map(Into::into)?,
            subscription_status: row.get::<_, u8>("subscription_status").map(Into::into)?,
            subscription_start: row
                .get::<_, i64>("subscription_start")
                .map(|t| chrono::DateTime::from_timestamp(t, 0).expect("invalid timestamp."))
                .ok(),
            subscription_end: row
                .get::<_, i64>("subscription_end")
                .map(|t| chrono::DateTime::from_timestamp(t, 0).expect("invalid timestamp."))
                .ok(),
            last_used: row
                .get::<_, i64>("last_used")
                .map(|t| chrono::DateTime::from_timestamp(t, 0).expect("invalid timestamp."))
                .ok(),
        })
    }
}

impl NfcCard {
    pub fn new(
        uid: CardID,
        username: impl Into<Arc<str>>,
        membership_id: usize,
        subscription_tier: SubTier,
        subscription_status: SubStatus,
        subscription_start: chrono::DateTime<chrono::Utc>,
        subscription_end: chrono::DateTime<chrono::Utc>,
        last_used: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            membership_id,
            uid,
            subscription_tier,
            subscription_status,
            username: username.into(),
            subscription_start: Some(subscription_start),
            subscription_end: Some(subscription_end),
            last_used: Some(last_used),
        }
    }

    pub const fn membership_id(&self) -> usize {
        self.membership_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub const fn subscription_start(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.subscription_start.as_ref()
    }

    pub const fn subscription_end(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.subscription_end.as_ref()
    }

    pub const fn last_used(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.last_used.as_ref()
    }

    pub const fn uid(&self) -> CardID {
        self.uid
    }

    pub const fn subscription_tier(&self) -> SubTier {
        self.subscription_tier
    }

    pub const fn subscription_status(&self) -> SubStatus {
        self.subscription_status
    }
}

pub struct NfcCardBuilder {
    username: Option<Arc<str>>,
    membership_id: Option<usize>,
    subscription_start: Option<chrono::DateTime<chrono::Utc>>,
    subscription_end: Option<chrono::DateTime<chrono::Utc>>,
    last_used: Option<chrono::DateTime<chrono::Utc>>,
    uid: Option<CardID>,
    subscription_tier: Option<SubTier>,
    subscription_status: Option<SubStatus>,
}

impl NfcCardBuilder {
    pub const fn new() -> Self {
        Self {
            username: None,
            membership_id: None,
            subscription_start: None,
            subscription_end: None,
            last_used: None,
            uid: None,
            subscription_tier: None,
            subscription_status: None,
        }
    }

    pub fn username(mut self, username: impl Into<Arc<str>>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub const fn membership_id(mut self, membership_id: usize) -> Self {
        self.membership_id = Some(membership_id);
        self
    }

    pub const fn start(mut self, start: isize) -> Self {
        self.subscription_start = Some(
            chrono::DateTime::from_timestamp(start as i64, 0).expect("invalid start timestamp."),
        );
        self
    }

    pub const fn end(mut self, end: isize) -> Self {
        self.subscription_end = Some(
            chrono::DateTime::from_timestamp(end as i64, 0).expect("invalid expiry timestamp."),
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

    pub const fn subscription_tier(mut self, tier: SubTier) -> Self {
        self.subscription_tier = Some(tier);
        self
    }

    pub const fn subscription_status(mut self, status: SubStatus) -> Self {
        self.subscription_status = Some(status);
        self
    }

    #[inline]
    pub fn finish(self) -> NfcCard {
        NfcCard {
            username: self.username.unwrap_or_default(),
            membership_id: self.membership_id.unwrap_or_default(),
            subscription_start: self.subscription_start,
            subscription_end: self.subscription_end,
            last_used: self.last_used,
            uid: self.uid.unwrap_or_else(CardID::new_zeroed),
            subscription_tier: self.subscription_tier.unwrap_or_default(),
            subscription_status: self.subscription_status.unwrap_or_default(),
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
