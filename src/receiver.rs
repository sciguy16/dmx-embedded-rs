use crate::transport::Transport;
use crate::UNIVERSE_CHANNELS;

/// Receiver
pub struct Receiver<TRANSPORT> {
    rx_buffer: [u8; UNIVERSE_CHANNELS],
    transport: TRANSPORT,
}

impl<TRANSPORT> Receiver<TRANSPORT>
where
    TRANSPORT: Transport,
{
    /// Get the current values of the entire universe
    pub fn get_values(&self) -> &[u8; UNIVERSE_CHANNELS] {
        &self.rx_buffer
    }

    /// Get a specific channel value. Panics if `chan` > `UNIVERSE_CHANNELS`
    pub fn get_channel(&self, chan: usize) -> u8 {
        self.rx_buffer[chan]
    }

    /// Initialise a receiver from an embedded-hal UART
    pub fn from_uart(uart: ()) -> Self {
        todo!()
    }
}
