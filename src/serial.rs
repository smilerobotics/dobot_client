use crate::protocol::*;
use crate::traits::Device;
use failure::Error;
use serial::{SerialPort, SerialPortSettings, SystemPort};
use std::io::{Read, Write};
use std::path::Path;

pub struct SerialDevice {
    device: SystemPort,
}

impl SerialDevice {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, serial::Error> {
        let mut device = serial::open(path.as_ref())?;
        device.reconfigure(&|settings: &mut dyn SerialPortSettings| {
            settings.set_baud_rate(serial::Baud115200)?;
            settings.set_char_size(serial::Bits8);
            settings.set_stop_bits(serial::Stop1);
            settings.set_parity(serial::ParityNone);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;
        device.set_timeout(std::time::Duration::from_millis(1000))?;
        Ok(Self { device })
    }
}

impl Device for SerialDevice {
    fn send(&mut self, packet: PayloadStruct) -> Result<PayloadStruct, Error> {
        let send_buf = packet.serialize();
        self.device.write_all(&send_buf)?;
        self.device.flush()?;
        let mut header_buf = [1; 3];
        self.device.read_exact(&mut header_buf)?;
        let len = check_header_and_get_payload_size(&header_buf)?;
        let mut buf_remaining = Vec::new();
        buf_remaining.resize(len + 3, 1);
        self.device.read_exact(&mut buf_remaining)?;

        let mut final_buf = vec![];
        final_buf.push(header_buf[0]);
        final_buf.push(header_buf[1]);
        final_buf.push(header_buf[2]);
        final_buf.append(&mut buf_remaining);
        PayloadStruct::deserialize(&final_buf)
    }
}
