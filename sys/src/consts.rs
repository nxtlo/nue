pub const MODULATIONS: &[nfc1::Modulation] = &[
    nfc1::Modulation {
        modulation_type: nfc1::ModulationType::Iso14443a,
        baud_rate: nfc1::BaudRate::Baud106,
    },
    nfc1::Modulation {
        modulation_type: nfc1::ModulationType::Iso14443b,
        baud_rate: nfc1::BaudRate::Baud106,
    },
];

pub const PAGE_SIZE: usize = 4;
pub const USER_PAGE_END: u8 = 0x81;
pub const USER_PAGE_START: u8 = 0x04;
pub const WRITE_CMD: u8 = 0xA2;
pub const READ_CMD: u8 = 0x30;

pub const CARD_SIZE: usize = size_of::<super::NfcCard>();
pub const PAGES_NEEDED: usize = CARD_SIZE.div_ceil(PAGE_SIZE);
pub const PADDED_SIZE: usize = PAGES_NEEDED * PAGE_SIZE;
pub const READS_NEEDED: usize = PAGES_NEEDED.div_ceil(4); // READ returns 4 pages per call
pub const RX_BUF_SIZE: usize = READS_NEEDED * 16;

pub const _FITS: () = assert!(
    PAGES_NEEDED <= (USER_PAGE_END - USER_PAGE_START + 1) as usize,
    "NfcCard too large for NTAG215"
);
