pub mod address_mode;
pub mod define;
pub mod label;
pub mod user_count;

pub type Address = u16;

pub trait ZeroPage {
    fn is_zeropage(&self) -> bool;
    fn low(&self) -> u8;
    fn high(&self) -> u8;
}

impl ZeroPage for Address {
    fn is_zeropage(&self) -> bool {
        *self < 0x100
    }

    fn low(&self) -> u8 {
        (self & 0xFF) as u8
    }

    fn high(&self) -> u8 {
        (self >> 8) as u8
    }
}
