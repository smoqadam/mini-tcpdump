use std::net::IpAddr;

use crate::{ parser::{ ParsedPacket}, ParsedTransport, Args };
use serde::{Serialize};


#[derive(Debug, Serialize)]
pub struct PacketFilter {
    pub protocol: Option<Protocol>,
    pub src_port: Option<u16>,
    pub dest_port: Option<u16>,
    pub src_host: Option<IpAddr>,
    pub dest_host: Option<IpAddr>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Serialize)]
pub enum Protocol {
    Tcp,
    Udp,
    Http,
}

impl PacketFilter {
    pub fn from_args(args: &Args) -> PacketFilter {
        PacketFilter {
            protocol: args.protocol,
            src_port: args.src_port,
            dest_port: args.dest_port,
            src_host: args.src_host,
            dest_host: args.dest_host,
        }
    }

    pub fn matches(&self, packet: &ParsedPacket) -> bool {
        return 
            self.match_protocol(packet) &&
            self.match_dest_port(packet) &&
            self.match_dst_host(packet) &&
            self.match_src_host(packet) &&
            self.match_src_port(packet)
        ;
    }

    fn match_protocol(&self, packet: &ParsedPacket) -> bool {
        match self.protocol {
            Some(Protocol::Tcp) => matches!(packet.transport, Some(ParsedTransport::Tcp(..))),
            Some(Protocol::Udp) => matches!(packet.transport, Some(ParsedTransport::Udp(..))),
            Some(Protocol::Http) => matches!(packet.transport, Some(ParsedTransport::Tcp(..))),
            None => true,
        }
    }

    fn match_dest_port(&self, packet: &ParsedPacket) -> bool {
        match self.dest_port {
            Some(port) => {
                if let Some(t) = &packet.transport { t.dst_port() == Some(port) } else { false }
            }
            None => true,
        }
    }

    fn match_src_port(&self, packet: &ParsedPacket) -> bool {
        match self.src_port {
            Some(port) => {
                if let Some(t) = &packet.transport { t.src_port() == Some(port) } else { false }
            }
            None => true,
        }
    }

    fn match_src_host(&self, packet: &ParsedPacket) -> bool {
        match self.src_host {
            Some(ip) => {
                if let Some(t) = &packet.network { t.src_host() == Some(ip) } else { false }
            }
            None => true,
        }
    }

    fn match_dst_host(&self, packet: &ParsedPacket) -> bool {
        match self.dest_host {
            Some(ip) => {
                if let Some(t) = &packet.network { t.dst_host() == Some(ip) } else { false }
            }
            None => true,
        }
    }
}
