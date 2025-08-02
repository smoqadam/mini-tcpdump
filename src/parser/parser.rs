use pnet::packet::{
    ethernet::{ EtherTypes, EthernetPacket },
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::TcpPacket,
    Packet,
};

use crate::parser::{ ParsedEthernet, ParsedIp, ParsedTransport, ParsedPacket };

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

fn parse_ipv4(ipv4: Ipv4Packet) -> Option<ParsedTransport> {
    let transport = match ipv4.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => parse_tcp(ipv4.payload()),
        _ => { Some(ParsedTransport::Unknown) }
    };

    transport
}

fn parse_tcp(packet: &[u8]) -> Option<ParsedTransport> {
    let tcp = TcpPacket::new(packet)?;
    Some(ParsedTransport::Tcp {
        src_port: tcp.get_source(),
        dest_port: tcp.get_destination(),
        payload: tcp.payload().to_vec(),
    })
}

fn parse_ipv6(ipv6: Ipv6Packet) -> Option<ParsedTransport> {
    let transport = match ipv6.get_next_header() {
        IpNextHeaderProtocols::Tcp => parse_tcp(ipv6.payload()),
        _ => { Some(ParsedTransport::Unknown) }
    };
    transport
}
