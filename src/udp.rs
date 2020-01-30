// This is not working for now, i don't know why...
use crate::protocol::PayloadStruct;
use crate::traits::*;
use std::net::{IpAddr, UdpSocket};

pub struct UdpDevice {
    udp_socket: UdpSocket,
}

impl UdpDevice {
    pub fn new(ip: IpAddr) -> Result<Self, std::io::Error> {
        let udp_socket = UdpSocket::bind("192.168.1.10:8889")?;
        udp_socket.connect((ip, 54321))?;
        let duration = std::time::Duration::new(1, 0);
        let dur = Option::Some(duration);
        let _res = udp_socket
            .set_read_timeout(dur)
            .expect("failed to set timeout");

        Ok(Self { udp_socket })
    }
}

impl Device for UdpDevice {
    fn send(&mut self, packet: PayloadStruct) -> Result<PayloadStruct, failure::Error> {
        let buf = packet.serialize();
        self.udp_socket.send(&buf)?;
        println!("send");
        let mut buf = [0; 128];
        let _size = self.udp_socket.recv(&mut buf)?;
        println!("recv");
        PayloadStruct::deserialize(&buf)
    }
}
