pub mod parser;
pub mod ip;
pub mod protocol;

use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr };
use serde::{ Serialize  ,Serializer};

use pnet::{ packet::{ ethernet::{ EtherType }, ip::IpNextHeaderProtocol }, util::MacAddr };

#[derive(Debug, Clone, Serialize)]
pub struct ParsedPacket {
    pub ethernet: ParsedEthernet,
    pub network: Option<ParsedNetwork>,
    pub transport: Option<ParsedTransport>,
    // pub application: Option<ParsedApplication>, // todo later
}



fn mac_serialize<S>(x: &MacAddr, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(format!("{}", x).as_str())
}


#[derive(Debug, Clone, Serialize)]
pub struct ParsedEthernet {
    #[serde(serialize_with = "mac_serialize")]
    pub src_mac: MacAddr,
    #[serde(serialize_with = "mac_serialize")]
    pub dest_mac: MacAddr,
    // pub ether_type: EtherType,
}


#[derive(Debug, Clone, Serialize)]
pub enum ParsedNetwork {
    Ipv4(Ipv4Info),
    Ipv6(Ipv6Info),
    Unknown,
}

impl ParsedNetwork {
    pub fn src_host(&self) -> Option<IpAddr> {
        match self {
            ParsedNetwork::Ipv4(ip) => Some(IpAddr::V4(ip.src)),
            ParsedNetwork::Ipv6(ip) => Some(IpAddr::V6(ip.src)),
            ParsedNetwork::Unknown => None,
        }
    }

    pub fn dst_host(&self) -> Option<IpAddr> {
        match self {
            ParsedNetwork::Ipv4(ip) => Some(IpAddr::V4(ip.dest)),
            ParsedNetwork::Ipv6(ip) => Some(IpAddr::V6(ip.dest)),
            ParsedNetwork::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Ipv6Info {
    pub src: Ipv6Addr,
    pub dest: Ipv6Addr,
    // proto: IpNextHeaderProtocol,
}

#[derive(Debug, Clone, Serialize)]
pub struct Ipv4Info {
    pub src: Ipv4Addr,
    pub dest: Ipv4Addr,
    // proto: IpNextHeaderProtocol,
}

#[derive(Debug, Clone, Serialize)]
pub enum ParsedTransport {
    Tcp(TcpInfo),
    Udp(UdpInfo),
    Unknown,
}
#[derive(Debug, Clone, Serialize)]
pub struct TcpInfo {
    pub src_port: u16,
    pub dest_port: u16,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
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
