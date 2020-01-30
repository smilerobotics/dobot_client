use crate::protocol::PayloadStruct;
use failure::Error;

pub trait Device {
    fn send(&mut self, packet: PayloadStruct) -> Result<PayloadStruct, Error>;
}
