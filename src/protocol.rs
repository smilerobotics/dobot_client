use failure::format_err;
use failure::Error;
use std::fmt;
use std::num::Wrapping;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum ReadWrite {
    READ,
    WRITE,
}

impl From<u8> for ReadWrite {
    fn from(rw: u8) -> ReadWrite {
        if rw == 1 {
            ReadWrite::WRITE
        } else {
            ReadWrite::READ
        }
    }
}

#[derive(Clone)]
pub struct PayloadStruct {
    pub id: u8,
    pub rw: ReadWrite,
    pub is_queued: bool,
    pub params: Vec<u8>,
}

impl fmt::Debug for PayloadStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, rw: {:?}, queued: {}, params:{}",
            self.id,
            self.rw,
            self.is_queued,
            String::from_utf8(self.params.clone()).unwrap()
        )
    }
}

fn calc_complement(val: u8) -> u8 {
    (256 - val as usize) as u8
}

#[test]
fn test_calc_complement() {
    assert_eq!(calc_complement(1), 255);
    assert_eq!(calc_complement(calc_complement(233)), 233);
}

fn calc_sum(vals: &[u8]) -> u8 {
    vals.iter()
        .map(|v| Wrapping(*v))
        .fold(Wrapping(0u8), |sum, v| sum + v)
        .0
}

fn calc_checksum(vals: &[u8]) -> u8 {
    calc_complement(calc_sum(vals))
}

pub(crate) fn check_header_and_get_payload_size(packet: &[u8]) -> Result<usize, Error> {
    if packet[0] != 0xAA || packet[1] != 0xAA {
        return Err(format_err!(
            "header is invalid [{}], [{}]",
            packet[0],
            packet[1]
        ));
    }
    if packet[2] < 2 {
        return Err(format_err!(
            "packet invalid, packet[2] must be greater than 2 but it is {}",
            packet[2]
        ));
    }
    Ok(packet[2] as usize - 2)
}

impl Default for PayloadStruct {
    fn default() -> Self {
        Self::new()
    }
}

impl PayloadStruct {
    pub fn new() -> Self {
        Self::with_id(0)
    }

    pub fn with_id(id: u8) -> Self {
        PayloadStruct {
            id: id,
            rw: ReadWrite::READ,
            is_queued: false,
            params: vec![],
        }
    }

    pub fn set_write(mut self) -> Self {
        self.rw = ReadWrite::WRITE;
        self
    }

    pub fn set_queued(mut self) -> Self {
        self.is_queued = true;
        self
    }

    pub fn set_params(mut self, params: Vec<u8>) -> Self {
        self.params = params;
        self
    }

    pub fn serialize(mut self) -> Vec<u8> {
        let mut payload = vec![self.id];
        let rw_bit = self.rw as u8;
        let queue_bit = if self.is_queued { 0b10 } else { 0b00 };
        let ctrl = rw_bit | queue_bit;
        let len = (self.params.len() + 2) as u8;

        payload.push(ctrl);
        payload.append(&mut self.params);
        let sum = calc_checksum(&payload);

        let mut packet = vec![0xAA, 0xAA];
        packet.push(len);
        packet.append(&mut payload);
        packet.push(sum);
        packet
    }

    pub fn deserialize(packet: &[u8]) -> Result<Self, Error> {
        let mut data = PayloadStruct::new();
        let len = check_header_and_get_payload_size(packet)?;
        // header(0xAA, 0xAA=2) + len(1)
        const CHECKSUM_START: usize = 3;
        // + id(1) + ctrl(1)
        const PAYLODAD_START: usize = CHECKSUM_START + 2;
        let payload_end = (PAYLODAD_START + len) as usize;
        if packet.len() < (payload_end + 1) as usize {
            return Err(format_err!(
                "packet length should be {}, but actual packet size {}",
                payload_end + 1,
                packet.len()
            ));
        }
        data.id = packet[3];
        data.rw = (0b1 & packet[4]).into();
        data.is_queued = 0b10 & packet[4] != 0b0;
        data.params = packet[PAYLODAD_START..payload_end].to_vec();
        let sum = calc_checksum(&packet[CHECKSUM_START..payload_end]);
        if sum != packet[payload_end] {
            return Err(format_err!(
                "checksum error: sum should be {}, but calculation result is {}",
                packet[payload_end],
                sum
            ));
        }
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serialize1() {
        let mut p1 = PayloadStruct::new();
        p1.id = 2;
        p1.is_queued = true;
        let buf = p1.serialize();
        let p2 = PayloadStruct::deserialize(&buf).unwrap();
        assert_eq!(p2.id, 2);
        assert_eq!(p2.is_queued, true);
        assert_eq!(p2.rw, ReadWrite::READ);
        assert_eq!(p2.params.len(), 0);
    }

    #[test]
    fn serialize2() {
        let mut p1 = PayloadStruct::new();
        p1.id = 3;
        p1.is_queued = false;
        p1.rw = ReadWrite::WRITE;
        p1.params = vec![97, 98, 99, 100];
        let buf = p1.serialize();
        let p2 = PayloadStruct::deserialize(&buf).unwrap();
        assert_eq!(p2.id, 3);
        assert_eq!(p2.is_queued, false);
        assert_eq!(p2.rw, ReadWrite::WRITE);
        assert_eq!(p2.params.len(), 4);
        assert_eq!(p2.params[0], 97);
        assert_eq!(p2.params[1], 98);
        assert_eq!(p2.params[2], 99);
        assert_eq!(p2.params[3], 100);
        println!("{:?}", p2);
    }
    #[test]
    fn test_queued1() {
        let p = PayloadStruct::with_id(1).set_queued();
        let buf = p.serialize();
        assert_eq!(buf[4], 2);
    }
    #[test]
    fn test_queued2() {
        let p = PayloadStruct::with_id(1).set_queued().set_write();
        let buf = p.serialize();
        assert_eq!(buf[4], 3);
    }

    #[test]
    fn test_calc_sum() {
        assert_eq!(calc_sum(&vec![10, 100]), 110);
        assert_eq!(calc_sum(&vec![200, 200]), 144);
    }
}
