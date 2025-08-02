use pnet::packet::{ethernet::{EtherTypes, EthernetPacket}, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, tcp::TcpPacket, Packet};

use crate::parser::{ParsedEthernet, ParsedIp, ParsedTransport, ParsedPacket};

pub fn parse(packet: &[u8]) -> Option<ParsedPacket> {

    let eth_packet = EthernetPacket::new(packet)?;

    let ethernet = ParsedEthernet {
        src_mac: eth_packet.get_source(),
        dest_mac: eth_packet.get_destination(),
        ether_type: eth_packet.get_ethertype()
    };


    let mut ip = None;
    let mut transport = None;
    match eth_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            if let Some(ipv4) = Ipv4Packet::new(eth_packet.payload()) {
                ip = Some(ParsedIp::V4 {
                    src: ipv4.get_source(),
                    dest: ipv4.get_destination(),
                    proto: ipv4.get_next_level_protocol(),
                });

                match ipv4.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                            transport = Some(ParsedTransport::Tcp {
                                src_port: tcp.get_source(),
                                dest_port: tcp.get_destination(),
                                payload: tcp.payload().to_vec(),
                            });
                        }
                    }
                    _ => {}
                }
            }
        },

        _ => {}
       
    }

    Some(ParsedPacket{
        ethernet, ip, transport
    })

}