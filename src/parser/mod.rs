pub mod parser;
pub mod ip;
pub mod protocol;

use std::net::{ Ipv4Addr, Ipv6Addr };

use pnet::{ packet::{ ethernet::{ EtherType }, ip::IpNextHeaderProtocol }, util::MacAddr };

#[derive(Debug, Clone)]
pub struct ParsedPacket {
    pub ethernet: ParsedEthernet,
    pub network: Option<ParsedNetwork>,
    pub transport: Option<ParsedTransport>,
    // pub application: Option<ParsedApplication>, // todo later
}

#[derive(Debug, Clone)]
pub struct ParsedEthernet {
    pub src_mac: MacAddr,
    pub dest_mac: MacAddr,
    pub ether_type: EtherType,
}

#[derive(Debug, Clone)]
pub enum ParsedNetwork {
    Ipv4(Ipv4Info),
    Ipv6(Ipv6Info),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Ipv6Info {
    src: Ipv6Addr,
    dest: Ipv6Addr,
    proto: IpNextHeaderProtocol,
}

#[derive(Debug, Clone)]
pub struct Ipv4Info {
    src: Ipv4Addr,
    dest: Ipv4Addr,
    proto: IpNextHeaderProtocol,
}

#[derive(Debug, Clone)]
pub enum ParsedTransport {
    Tcp(TcpInfo),
    Udp(UdpInfo),
    Unknown,
}
#[derive(Debug, Clone)]
pub struct TcpInfo {
    pub src_port: u16,
    pub dest_port: u16,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct UdpInfo {
    pub src_port: u16,
    pub dest_port: u16,
    pub payload: Vec<u8>,
}

pub trait HasPorts {
    fn src_port(&self) -> u16;
    fn dst_port(&self) -> u16;
}

impl HasPorts for TcpInfo {
    fn src_port(&self) -> u16 {
        self.src_port
    }
    fn dst_port(&self) -> u16 {
        self.dest_port
    }
}


impl HasPorts for UdpInfo {
    fn src_port(&self) -> u16 {
        self.src_port
    }
    fn dst_port(&self) -> u16 {
        self.dest_port
    }
}


impl ParsedTransport {
    pub fn src_port(&self) -> Option<u16> {
        match self {
            ParsedTransport::Tcp(info) => Some(info.src_port()),
            ParsedTransport::Udp(info) => Some(info.src_port()),
            ParsedTransport::Unknown => None,
        }
    }

    pub fn dst_port(&self) -> Option<u16> {
        match self {
            ParsedTransport::Tcp(info) => Some(info.dst_port()),
            ParsedTransport::Udp(info) => Some(info.dst_port()),
            ParsedTransport::Unknown => None,
        }
    }
}