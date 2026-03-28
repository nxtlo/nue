#[cfg(test)]
extern crate std;

mod consts;
pub mod iter;

use core::fmt;

use nfc1::target_info::TargetInfo;

//re-exports.
pub use crate::iter::Incoming;
pub use nue_model::{
    auth::Token,
    raw_card::{CardID, RawCard},
};

pub struct App<'a> {
    pub(crate) device: nfc1::Device<'a>,
}

impl<'a> App<'a> {
    pub fn new(mut device: nfc1::Device<'a>) -> nfc1::Result<Self> {
        device.initiator_init()?;
        Ok(Self { device: device })
    }

    /// Opens an NFC device using the UART connection string `pn532_uart:/dev/ttyS0:115200`.
    pub fn uart(ctx: &'a mut nfc1::Context) -> nfc1::Result<Self> {
        // TODO: Make this configurable.
        Self::new(ctx.open_with_connstring("pn532_uart:/dev/ttyS0:115200")?)
    }

    /// Creates a new `nfc1::Context`.
    #[inline]
    pub fn context() -> nfc1::Result<nfc1::Context<'a>> {
        nfc1::Context::new()
    }

    #[inline]
    pub fn device_name(&mut self) -> &str {
        self.device.name()
    }

    /// Consumes `self` and returns the underlying `nfc1::Device`.
    #[inline]
    pub fn into_device(self) -> nfc1::Device<'a> {
        self.device
    }

    /// Returns a mutable reference to the underlying `nfc1::Device`.
    #[inline]
    pub const fn device(&'a mut self) -> &'a mut nfc1::Device<'a> {
        &mut self.device
    }

    /// Returns an iterator over incoming cards.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use nue_sys::App;
    ///
    /// let mut ctx = App::context()?;
    /// let mut app = App::uart(&mut ctx)?;
    ///
    /// for card in app.incoming() {
    ///     ...
    /// }
    /// ```
    #[inline]
    #[must_use = "Iterators are lazy and do nothing unless consumed."]
    pub fn incoming(&mut self) -> Incoming<'a, '_> {
        Incoming(self)
    }

    /// Polls for a single target once, returning the card ID and raw card data if found.
    ///
    /// This function does not block and wait for a card to be found. If you want to achieve that behavior,
    /// you should call this function in a loop until it returns `Ok((_, _))`, Or use [`App::incoming`] instead.
    ///
    /// You can pass a specific modulation to use, or `None` to use the default modulation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nue_sys::App;
    ///
    /// let mut ctx = App::context()?;
    /// let mut app = App::uart(&mut ctx)?;
    ///
    /// if let Ok((card_id, raw_card)) = app.poll_once(None) {
    ///     println!("Card ID: {:?}, Raw Card: {:?}", card_id, raw_card);
    /// }
    /// ```
    pub fn poll_once(
        &mut self,
        modulation: Option<nfc1::Modulation>,
    ) -> nue_model::Result<(CardID, RawCard)> {
        let modulation = modulation.unwrap_or(consts::MODULATIONS[0]);

        // Scan for a card.
        let target = self
            .device
            .initiator_select_passive_target(&modulation)
            .map_err(|_| nue_model::Error::NfcReadError)?;

        // Extract the UID from the target info, currently only ISO14443a is supported.
        let uid = match target.target_info {
            TargetInfo::Iso14443a(iso) => iso.uid,
            _ => return Err(nue_model::Error::NfcCardUnrecognized),
        };

        let raw = self.read().map_err(|_| nue_model::Error::NfcReadError)?;

        self.device
            .initiator_deselect_target()
            .map_err(|_| nue_model::Error::NfcReadError)?;

        return Ok((uid.into(), raw));
    }

    // TODO: Probably needs a re-write.
    /// Read the raw bytes inside the card from the NFC device, turning it into a [`RawCard`].
    pub fn read(&mut self) -> nfc1::Result<RawCard> {
        let mut rx = [0u8; consts::RX_BUF_SIZE];

        for i in 0..consts::READS_NEEDED {
            let page = consts::USER_PAGE_START + (i * 4) as u8;
            let cmd = [consts::READ_CMD, page];
            let chunk: [u8; 16] = self
                .device
                .initiator_transceive_bytes(&cmd, 16, nfc1::Timeout::Default)?
                .try_into()
                .map_err(|_| nfc1::Error::Soft)?;

            unsafe {
                let dest_ptr = rx.as_mut_ptr().add(i * 16);
                let src_ptr = chunk.as_ptr();
                core::ptr::copy_nonoverlapping(src_ptr, dest_ptr, 16);
            }
        }

        let card = RawCard::from_bytes(&rx[..consts::CARD_SIZE]).ok_or(nfc1::Error::Soft)?;
        Ok(*card)
    }

    // TODO: Probably needs a re-write.
    fn write_page(&mut self, page: u8, data: &[u8; 4]) -> nfc1::Result<()> {
        debug_assert!(
            page <= consts::USER_PAGE_END,
            "page {:#X} is outside NTAG215 user memory",
            page
        );

        let cmd = [consts::WRITE_CMD, page, data[0], data[1], data[2], data[3]];
        self.device
            .initiator_transceive_bytes(&cmd, 1, nfc1::Timeout::Default)?;
        Ok(())
    }

    // TODO: Probably needs a re-write.
    pub fn write(&mut self, card: &RawCard) -> nfc1::Result<()> {
        let mut padded = [0u8; consts::PADDED_SIZE];
        unsafe {
            let dest_ptr = padded.as_mut_ptr();
            let src_ptr = card.as_slice().as_ptr();
            // We need to use consts::CARD_SIZE here as that's the length of the slice
            core::ptr::copy_nonoverlapping(src_ptr, dest_ptr, consts::CARD_SIZE);
        }

        for (i, chunk) in padded.chunks_exact(consts::PAGE_SIZE).enumerate() {
            let page = consts::USER_PAGE_START + i as u8;
            self.write_page(page, chunk.try_into().unwrap())?;
        }
        Ok(())
    }
}

impl fmt::Debug for App<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("device", &core::ptr::addr_of!(self.device))
            .finish()
    }
}

// allow tests for raspberry pi only
#[cfg(all(test, unix, target_pointer_width = "32"))]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read() -> nfc1::Result<()> {
        let mut ctx = App::context()?;
        let app = App::uart(&mut ctx)?;
        dbg!(app);
        Ok(())
    }
}
