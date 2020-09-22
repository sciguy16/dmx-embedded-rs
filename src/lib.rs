#![no_std]

#[cfg(feature = "receiver")]
pub mod receiver;
#[cfg(feature = "transmitter")]
pub mod transmitter;

pub mod transport;

const UNIVERSE_CHANNELS: usize = 512;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
