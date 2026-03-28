use core::fmt::Debug;

use crate::App;
use crate::consts;
use nue_model::raw_card::{CardID, RawCard};
use nue_model::{Error, Result};

use nfc1::target_info::TargetInfo;

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
        let poll_result =
            self.0
                .device
                .initiator_poll_target(consts::MODULATIONS, 20, consts::POLL_MS);

        let target = match poll_result {
            Ok(t) => t,
            // You might want to distinguish between "No Card" and "Hardware Error" here
            Err(_) => return Some(Err(Error::NfcCardUnrecognized)),
        };

        let TargetInfo::Iso14443a(inner) = target.target_info else {
            return Some(Err(Error::NfcCardUnrecognized));
        };

        let uid = inner.uid;

        let read_res = self.0.read().map_err(|_| Error::NfcReadError);
        let _ = self.0.device.initiator_deselect_target();

        match read_res {
            Ok(raw_card) => Some(Ok((uid.into(), raw_card))),
            Err(e) => Some(Err(e)),
        }
    }
}
