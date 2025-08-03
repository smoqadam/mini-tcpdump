mod parser;
mod sniffer;
mod filter;
use std::net::IpAddr;

use clap::Parser;

use crate::{ filter::filter::{ PacketFilter, Protocol }, parser::{ ParsedTransport } };

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interface: String,

    #[arg(short, long, value_enum)]
    protocol: Option<Protocol>,

    #[arg(long)]
    port: Option<u16>,

    #[arg[long = "src-port"]]
    src_port: Option<u16>,

    #[arg[long = "dst-port"]]
    dest_port: Option<u16>,

    #[arg[long = "src-host"]]
    src_host: Option<IpAddr>,
    #[arg[long = "dst-host"]]
    dest_host: Option<IpAddr>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let packet_filter = PacketFilter::from_args(&args);

    println!("{:?}", packet_filter);

    let callback = Box::new(move |network_packet: parser::ParsedPacket| {
        // let ParsedPacket { ethernet, ip, transport } = network_packet;

        if packet_filter.matches(network_packet.clone()) {
            println!("(packet: {:?})", &network_packet);
        }

        // println!(
        //     "Ethernet: {} -> {} ({:?})",
        //     ethernet.src_mac,
        //     ethernet.dest_mac,
        //     ethernet.ether_type
        // );

        // if let Some(ip) = ip {
        //     match ip {
        //         ParsedIp::V4 { src, dest, proto } => {
        //             println!("IPv4: {} -> {} (protocol: {:?})", src, dest, proto);
        //         }
        //         ParsedIp::V6 { src, dest, proto } => {
        //             println!("IPv6: {} -> {} (protocol: {:?})", src, dest, proto);
        //         }
        //         ParsedIp::Unknown => {
        //             println!("not implemented yet");
        //         }
        //     }
        // }

        // if let Some(transport) = transport {
        //     match transport {
        //         ParsedTransport::Tcp { src_port, dest_port, payload } => {
        //             println!(
        //                 "TCP: {} -> {} ({} bytes payload)",
        //                 src_port,
        //                 dest_port,
        //                 payload.len()
        //             );
        //         }
        //         ParsedTransport::Udp { src_port, dest_port, payload } => {
        //             println!(
        //                 "TCP: {} -> {} ({} bytes payload)",
        //                 src_port,
        //                 dest_port,
        //                 payload.len()
        //             );
        //         }
        //         ParsedTransport::Unknown {} => { println!("Unknown") }
        //     }
        // }
    });
    sniffer::sniff(args.interface, callback);
    Ok(())
}
