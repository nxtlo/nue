use core::error::Error;
use core::fmt::Debug;
use core::time::Duration;

use crate::App;
use crate::consts;
use nue_model::{card::CardID, card::NfcCard};

use nfc1::{Target, target_info::TargetInfo};

pub struct Incoming<'a, 'b>(pub(crate) &'b mut App<'a>);

impl Debug for Incoming<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("Incoming")
    }
}

impl<'a, 'b> Iterator for Incoming<'a, 'b> {
    type Item = Result<(CardID, NfcCard), Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        let target = match self.0.device.initiator_poll_target(
            consts::MODULATIONS,
            20,
            Duration::from_millis(300),
        ) {
            Ok(Target { target_info, .. }) => {
                let TargetInfo::Iso14443a(inner) = target_info else {
                    return None;
                };
                inner.uid
            }
            Err(e) => return Some(Err(e.into())),
        };

        let card = match self.0.read() {
            Ok(c) => c,
            Err(e) => return Some(Err(e.into())),
        };

        if let Err(e) = self.0.device.initiator_deselect_target() {
            return Some(Err(e.into()));
        }

        Some(Ok((CardID(target), card)))
    }
}
