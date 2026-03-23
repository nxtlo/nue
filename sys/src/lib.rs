mod consts;
mod iter;

use core::{error::Error, fmt};

use crate::iter::Incoming;
//re export.
pub use nue_model::{
    auth::Token,
    card::{CardID, NfcCard},
};

pub struct App<'a> {
    pub device: nfc1::Device<'a>,
}

impl<'a> App<'a> {
    pub fn new(mut device: nfc1::Device<'a>) -> Result<Self, Box<dyn Error + 'static>> {
        device.initiator_init()?;
        Ok(Self { device: device })
    }

    pub fn uart(ctx: &'a mut nfc1::Context) -> Result<Self, Box<dyn Error + 'static>> {
        Self::new(ctx.open_with_connstring("pn532_uart:/dev/ttyS0:115200")?)
    }

    pub fn context<'ctx>() -> Result<nfc1::Context<'ctx>, Box<dyn Error + 'static>> {
        nfc1::Context::new().map_err(|e| e.into())
    }

    pub fn device_name(&mut self) -> &str {
        self.device.name()
    }

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

    pub fn write(&mut self, card: &NfcCard) -> nfc1::Result<()> {
        let mut padded = [0u8; consts::PADDED_SIZE];
        padded[..consts::CARD_SIZE].copy_from_slice(card.as_slice());

        for (i, chunk) in padded.chunks_exact(consts::PAGE_SIZE).enumerate() {
            let page = consts::USER_PAGE_START + i as u8;
            self.write_page(page, chunk.try_into().unwrap())?;
        }
        Ok(())
    }

    pub fn read(&mut self) -> nfc1::Result<NfcCard> {
        let mut rx = [0u8; consts::RX_BUF_SIZE];

        for i in 0..consts::READS_NEEDED {
            let page = consts::USER_PAGE_START + (i * 4) as u8;
            let cmd = [consts::READ_CMD, page];
            let chunk: [u8; 16] = self
                .device
                .initiator_transceive_bytes(&cmd, 16, nfc1::Timeout::Default)?
                .try_into()
                .map_err(|_| nfc1::Error::Soft)?;
            rx[i * 16..(i + 1) * 16].copy_from_slice(&chunk);
        }

        let card = NfcCard::from_bytes(&rx[..consts::CARD_SIZE]).ok_or(nfc1::Error::Soft)?;
        Ok(*card)
    }

    pub fn into_device(self) -> nfc1::Device<'a> {
        self.device
    }

    pub const fn device(&'a mut self) -> &'a mut nfc1::Device<'a> {
        &mut self.device
    }

    pub fn incoming(&mut self) -> Incoming<'a, '_> {
        Incoming(self)
    }
}

impl fmt::Debug for App<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("device", &core::ptr::addr_of!(self.device))
            .finish()
    }
}
