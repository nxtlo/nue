extern crate alloc;

use alloc::collections::BTreeMap as Map;

use {
    crate::{Result, Storage},
    nue_model::{CardID, NfcCard},
};

#[derive(Clone, Debug, Default)]
pub struct Memory(Map<CardID, NfcCard>);
