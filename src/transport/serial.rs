use crate::transport::Transport;
use serialport::SerialPort;

pub struct SerialTransport<SERIALPORT> {
    port: SERIALPORT,
}

impl<SERIALPORT> SerialTransport<SERIALPORT>
where
    SERIALPORT: SerialPort,
{
    pub fn new(port: SERIALPORT) -> Self {
        Self { port }
    }
}

impl<SERIALPORT> Transport for SerialTransport<SERIALPORT>
where
    SERIALPORT: SerialPort,
{
    type Error = ();
    fn read_byte(&mut self) -> Result<char, Self::Error> {
        let mut buf = [0_u8];
        if let Ok(1) = self.port.read(&mut buf) {
            return Ok(buf[0] as char);
        }
        Err(())
    }
    fn write_byte(&self) -> Result<usize, <Self as Transport>::Error> {
        todo!()
    }
}
