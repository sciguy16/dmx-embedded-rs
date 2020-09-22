//! Abstraction over the possible transport types

use embedded_hal::serial::{Read, Write};

#[cfg(feature = "serial")]
pub mod serial;

#[cfg(feature = "embedded")]
pub mod embedded;

pub trait Serial: Read<char> + Write<char> {
    type Error;
    type BaudRate;
    fn set_baud_rate(
        baud: <Self as Serial>::BaudRate,
    ) -> Result<(), <Self as Serial>::Error>;
    // Probably don't need these fns because they're implied by the
    // Read+Write requirements
    /*fn read(&self) -> Result<u8, <Self as Serial>::Error> {
        self.read()
    }*/
    /*fn write(&self, data: &[u8]) -> Result<usize, <Self as Serial>::Error> {
        self.write(data)
    }*/
}

pub trait Transport {
    type Error;
    fn read_byte(&mut self) -> Result<char, Self::Error>;
    fn write_byte(&self) -> Result<usize, Self::Error>;
}
