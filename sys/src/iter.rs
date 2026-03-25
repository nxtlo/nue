use core::fmt::Debug;

use crate::App;
use crate::consts;
use nue_model::raw_card::{CardID, RawCard};
use nue_model::{Error, Result};

use nfc1::{Target, target_info::TargetInfo};

#[must_use = "Iterators are lazy and do nothing unless consumed."]
pub struct Incoming<'a, 'b>(pub(crate) &'b mut App<'a>);

// App is not `Debug`.
impl Debug for Incoming<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("Incoming")
    }
}

impl<'a, 'b> Iterator for Incoming<'a, 'b> {
    type Item = Result<(CardID, RawCard)>;

    fn next(&mut self) -> Option<Self::Item> {
        let uid =
            match self
                .0
                .device
                .initiator_poll_target(consts::MODULATIONS, 20, consts::POLL_MS)
            {
                // TODO: Handle the rest of the modulations, too lazy currently.
                // Also impl From<nfc1::Error> for nue_model::Error
                Ok(Target { target_info, .. }) => {
                    let TargetInfo::Iso14443a(inner) = target_info else {
                        return Some(Err(Error::NfcCardUnrecognized));
                    };
                    inner.uid
                }
                Err(_) => return Some(Err(Error::NfcCardUnrecognized)),
            };

        let Ok(raw_card) = self.0.read().map_err(|_| Error::NfcReadError) else {
            return Some(Err(Error::NfcReadError));
        };

        if let Ok(_) = self.0.device.initiator_deselect_target() {
            return Some(Ok((uid.into(), raw_card)));
        }

        None
    }
}
