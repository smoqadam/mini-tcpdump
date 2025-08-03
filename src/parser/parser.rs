use pnet::packet::{
    ethernet::{ EtherTypes, EthernetPacket },
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    Packet,
};

use crate::parser::{ ParsedEthernet, ParsedIp, ParsedTransport, ParsedPacket };
use crate::parser::ip::{ parse_ipv4, parse_ipv6 };
pub fn parse(packet: &[u8]) -> Option<ParsedPacket> {
    let eth_packet = EthernetPacket::new(packet)?;

    let ethernet = ParsedEthernet {
        src_mac: eth_packet.get_source(),
        dest_mac: eth_packet.get_destination(),
        ether_type: eth_packet.get_ethertype(),
    };

    let mut ip = None;

    let transport = match eth_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let ipv4 = Ipv4Packet::new(eth_packet.payload())?;
            ip = Some(ParsedIp::V4 {
                src: ipv4.get_source(),
                dest: ipv4.get_destination(),
                proto: ipv4.get_next_level_protocol(),
            });
            parse_ipv4(ipv4)
        }
        EtherTypes::Ipv6 => {
            let ipv6 = Ipv6Packet::new(eth_packet.payload())?;
            ip = Some(ParsedIp::V6 {
                src: ipv6.get_source(),
                dest: ipv6.get_destination(),
                proto: ipv6.get_next_header(),
            });
            parse_ipv6(ipv6)
        }
        _ => { Some(ParsedTransport::Unknown) }
    };

    Some(ParsedPacket {
        ethernet,
        ip,
        transport,
    })
}
