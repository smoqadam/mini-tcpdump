use crate::formatter::formatter::PacketFormatter;
use crate::parser::ParsedPacket;

pub struct Compact;

impl PacketFormatter for Compact {
    fn print(packet: ParsedPacket) -> String {
        let mut parts = Vec::new();
        
        parts.push(format!("{} > {}", packet.ethernet.src_mac, packet.ethernet.dest_mac));

        if let Some(network) = &packet.network {
            match network {
                crate::parser::ParsedNetwork::Ipv4(ipv4) => {
                    parts.push(format!("{} > {}", ipv4.src, ipv4.dest));
                },
                crate::parser::ParsedNetwork::Ipv6(ipv6) => {
                    parts.push(format!("{} > {}", ipv6.src, ipv6.dest));
                },
                crate::parser::ParsedNetwork::Unknown => {
                    parts.push("Unknown IP".to_string());
                }
            }
        }
        
        // Transport layer
        if let Some(transport) = &packet.transport {
            match transport {
                crate::parser::ParsedTransport::Tcp(tcp) => {
                    parts.push(format!("TCP:{}>{} ({} bytes)", tcp.src_port, tcp.dest_port, tcp.payload.len()));
                },
                crate::parser::ParsedTransport::Udp(udp) => {
                    parts.push(format!("UDP:{}>{} ({} bytes)", udp.src_port, udp.dest_port, udp.payload.len()));
                },
                crate::parser::ParsedTransport::Unknown => {
                    parts.push("Unknown Transport".to_string());
                }
            }
        }
        
        parts.join(" | ")
    }
}