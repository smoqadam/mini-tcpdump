use pnet::packet::{ ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, ipv6::Ipv6Packet };

use pnet::packet::{ Packet };

use crate::parser::ParsedTransport;
use crate::parser::protocol::{parse_tcp, parse_udp};


pub fn parse_ipv6(ipv6: Ipv6Packet) -> Option<ParsedTransport> {
    let transport = match ipv6.get_next_header() {
        IpNextHeaderProtocols::Tcp => parse_tcp(ipv6.payload()),
        _ => { Some(ParsedTransport::Unknown) }
    };
    transport
}

pub fn parse_ipv4(ipv4: Ipv4Packet) -> Option<ParsedTransport> {
    let transport = match ipv4.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => parse_tcp(ipv4.payload()),
        IpNextHeaderProtocols::Udp => parse_udp(ipv4.payload()),
        _ => { Some(ParsedTransport::Unknown) }
    };

    transport
}
