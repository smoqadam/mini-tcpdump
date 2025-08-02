use std::error::Error;
use std::fmt;
use std::net::IpAddr;
use std::rc::Rc;

use pnet::datalink::{ self, NetworkInterface };
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::{ Packet, MutablePacket };
use pnet::packet::ethernet::{ EtherTypes, EthernetPacket, MutableEthernetPacket };

#[derive(Debug)]
pub enum NetworkError {
    ParseFailed,
    InvalidPacketType,
}

impl Error for NetworkError {}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::ParseFailed => write!(f, "Parse failed"),
            NetworkError::InvalidPacketType => write!(f, "Invalid packet type"),
        }
    }
}

pub trait NetworkPacket {
    fn protocol(&self) -> &str;
    fn source(&self) -> String;
    fn destination(&self) -> String;
    fn summary(&self) -> String;
}



#[derive(Debug)]
pub struct IpV4Packet {
    pub src_mac: Option<String>,
    pub dest_mac: Option<String>,
    pub src_ip: Option<String>,
    pub dest_ip: Option<String>,
    pub protocol: Option<String>,
    pub flags: Option<u8>,
}

impl IpV4Packet {
    fn new(packet: &[u8]) -> IpV4Packet {
        unimplemented!()
    }

    fn parse(packet: &[u8]) {
        // parsing packet
    }

}
impl NetworkPacket for  IpV4Packet {
    fn protocol(&self) -> &str{unimplemented!()}
    fn source(&self) -> String {unimplemented!()}
    fn destination(&self) -> String{unimplemented!()}
    fn summary(&self) -> String{unimplemented!()}
}


pub struct Network {}
impl Network {
    pub fn new() -> Network {
        return Network {};
    }

    fn parse_packet(&self, packet: &[u8]) -> Result<Box<dyn NetworkPacket>, NetworkError> {
        let eth_packet = EthernetPacket::new(packet).ok_or(NetworkError::InvalidPacketType)?;
        let src_mac = eth_packet.get_source();
        let dest_mac = eth_packet.get_destination();
        match eth_packet.get_ethertype() {
            EtherTypes::Ipv4 => Ok(Box::new(IpV4Packet::new(eth_packet.payload()))),
            // EtherTypes::Ipv6 => {}
            _ => { Err(NetworkError::ParseFailed) }
        }
    }

    pub fn sniff(&self, interface: String, callback: Box<dyn Fn(Box<dyn NetworkPacket>)>) {
        let interface_name = interface;
        let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

        let interfaces = datalink::interfaces();
        let interface = interfaces.into_iter().filter(interface_names_match).next().unwrap();

        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
        };

        loop {
            match rx.next() {
                Ok(packet) => {
                    // let mut request = NetworkPacket { flags: None, dest: None, src: None, protocol: None, src_mac: None, dest_mac: None };
                    match self.parse_packet(packet) {
                        Ok(network_packet) => {
                            callback(network_packet);
                        }
                        Err(err) => {}
                    }
                }
                Err(e) => eprintln!("Error reading packet: {}", e),
            }
        }
    }
}
