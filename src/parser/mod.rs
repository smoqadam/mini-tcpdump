pub mod parser;
pub mod ip;
pub mod protocol;


use std::net::{Ipv4Addr, Ipv6Addr};

use pnet::{packet::{ethernet::{EtherType}, ip::IpNextHeaderProtocol}, util::MacAddr};



#[derive(Debug)]
pub struct ParsedPacket {
    pub ethernet: ParsedEthernet,
    pub ip: Option<ParsedIp>,
    pub transport: Option<ParsedTransport>,
}


#[derive(Debug)]
pub struct ParsedEthernet {
    pub src_mac: MacAddr,
    pub dest_mac: MacAddr,
    pub ether_type: EtherType,
}

#[derive(Debug)]
pub enum ParsedIp {
    Unknown,
    V4 {
        src: Ipv4Addr,
        dest: Ipv4Addr,
        proto: IpNextHeaderProtocol,
    },
    V6 {
        src: Ipv6Addr,
        dest: Ipv6Addr,
        proto: IpNextHeaderProtocol,
    },
}

#[derive(Debug)]
pub enum ParsedTransport {
    Unknown,
    Tcp {
        src_port: u16,
        dest_port: u16,
        payload: Vec<u8>,
    },
    Udp {
        src_port: u16,
        dest_port: u16,
        payload: Vec<u8>,
    },
}

