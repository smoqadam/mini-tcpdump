// use std::f32::consts::E;
// use std::fmt::Debug;
// use std::str::from_utf8;
mod parser;
mod sniffer;
use clap::Parser;

use crate::parser::{ParsedIp, ParsedTransport, ParsedPacket};

// use network::{ Network, NetworkPacket };

// use crate::network::NetworkError;

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interface: String,

    #[arg(short, long)]
    filter: Option<String>, //for filtering protocols tcp, http, etc. todo: use enum later

    #[arg(short, long)]
    port: u16,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    println!("interface {}!", args.interface);
    println!("port {}!", args.port);



    // let network = Network::new();
    // let filter = args.filter.unwrap().clone();
    let callback = Box::new(move |network_packet: parser::ParsedPacket| {

        let ParsedPacket { ethernet, ip, transport } = network_packet;
        // println!("Ethernet: {} -> {} ({:?})", ethernet.src_mac, ethernet.dest_mac, ethernet.ether_type);

        if let Some(ip) = ip {
            match ip {
                ParsedIp::V4 { src, dest, proto } => {
                    println!("IPv4: {} -> {} (protocol: {:?})", src, dest, proto);
                }
            }
        }

        if let Some(transport) = transport {
            match transport {
                ParsedTransport::Tcp { src_port, dest_port, payload } => {
                    println!("TCP: {} -> {} ({} bytes payload)", src_port, dest_port, payload.len());
                },
                ParsedTransport::Udp { src_port, dest_port, payload } => {
                    println!("TCP: {} -> {} ({} bytes payload)", src_port, dest_port, payload.len());

                }
            }
        }
        

        
    });
    sniffer::sniff(args.interface, callback);
    Ok(())
}
