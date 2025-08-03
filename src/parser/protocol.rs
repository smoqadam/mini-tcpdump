use pnet::packet::tcp::{TcpPacket};
use pnet::packet::udp::UdpPacket;
use pnet::packet::{
    Packet,
};
use crate::parser::ParsedTransport;

pub fn parse_tcp(packet: &[u8]) -> Option<ParsedTransport> {
    let tcp = TcpPacket::new(packet)?;
    Some(ParsedTransport::Tcp {
        src_port: tcp.get_source(),
        dest_port: tcp.get_destination(),
        payload: tcp.payload().to_vec(),
    })
}


pub fn parse_udp(packet: &[u8]) -> Option<ParsedTransport> {
    let udp = UdpPacket::new(packet)?;
    Some(ParsedTransport::Udp {
        src_port: udp.get_source(),
        dest_port: udp.get_destination(),
        payload: udp.payload().to_vec(),
    })
}