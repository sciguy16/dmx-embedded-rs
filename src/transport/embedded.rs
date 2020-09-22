use crate::transport::{Serial, Transport};

pub struct EmbeddedTransport<SERIALPORT> {
    port: SERIALPORT,
}

impl<SERIALPORT> EmbeddedTransport<SERIALPORT>
where
    SERIALPORT: Serial,
    //READ: Read<char>,
    //WRITE: Write<char>,
{
    pub fn new(port: SERIALPORT) -> Self {
        Self { port }
    }
}

impl<SERIALPORT> Transport for EmbeddedTransport<SERIALPORT>
where
    SERIALPORT: Serial,
{
    type Error = ();
    fn read_byte(&mut self) -> Result<char, Self::Error> {
        self.port.read().map_err(|_| ())
    }
    fn write_byte(&self) -> Result<usize, <Self as Transport>::Error> {
        todo!()
    }
}
