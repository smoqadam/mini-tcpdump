use pnet::packet::{
    ethernet::{ EtherTypes, EthernetPacket },
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    Packet,
};

use crate::parser::{
    Ipv4Info,
    Ipv6Info,
    ParsedEthernet,
    ParsedNetwork,
    ParsedPacket,
    ParsedTransport,
};
use crate::parser::ip::{ parse_ipv4, parse_ipv6 };
pub fn parse(packet: &[u8]) -> Option<ParsedPacket> {
    let eth_packet = EthernetPacket::new(packet)?;

    let ethernet = ParsedEthernet {
        src_mac: eth_packet.get_source(),
        dest_mac: eth_packet.get_destination(),
        ether_type: eth_packet.get_ethertype(),
    };

    let mut network = None;

    let transport = match eth_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let ipv4 = Ipv4Packet::new(eth_packet.payload())?;
            network = Some(
                ParsedNetwork::Ipv4(Ipv4Info {
                    src: ipv4.get_source(),
                    dest: ipv4.get_destination(),
                    proto: ipv4.get_next_level_protocol(),
                })
            );
            parse_ipv4(ipv4)
        }
        EtherTypes::Ipv6 => {
            let ipv6 = Ipv6Packet::new(eth_packet.payload())?;
            network = Some(
                ParsedNetwork::Ipv6(Ipv6Info {
                    src: ipv6.get_source(),
                    dest: ipv6.get_destination(),
                    proto: ipv6.get_next_header(),
                })
            );
            parse_ipv6(ipv6)
        }
        _ => { Some(ParsedTransport::Unknown) }
    };

    Some(ParsedPacket {
        ethernet,
        network,
        transport,
    })
}
