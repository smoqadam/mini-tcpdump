
use std::str::from_utf8;

use clap::Parser;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};




#[derive(Debug, Parser)]
#[command(version, about, long_about= None)]
struct Args {
    #[arg(short, long)]
    interface: String,

    #[arg(short, long)]
    filter: String, //for filtering protocols tcp, http, etc. todo: use enum later


    #[arg(short, long)]
    port: u8
}


fn main() {
    let args = Args::parse();
    println!("interface {}!", args.interface);
    println!("filter {}!", args.filter);
    println!("port {}!", args.port);


    let interface_name = args.interface;
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };
    loop {
        
match rx.next() {
    Ok(packet) => {
        if let Some(eth_packet) = EthernetPacket::new(packet) {
            println!(
                "Ethernet: {} -> {} (type: {:?})",
                eth_packet.get_source(),
                eth_packet.get_destination(),
                eth_packet.get_ethertype()
            );

            match eth_packet.get_ethertype() {
                EtherTypes::Ipv4 => {
                    if let Some(ip_packet) = Ipv4Packet::new(eth_packet.payload()) {
                        println!(
                            "IPv4: {} -> {} (protocol: {:?})",
                            ip_packet.get_source(),
                            ip_packet.get_destination(),
                            ip_packet.get_next_level_protocol()
                        );

                        match ip_packet.get_next_level_protocol() {
                            IpNextHeaderProtocols::Tcp => {
                                if let Some(tcp_packet) = TcpPacket::new(ip_packet.payload()) {
                                    println!(
                                        "TCP: {}:{} -> {}:{}",
                                        ip_packet.get_source(),
                                        tcp_packet.get_source(),
                                        ip_packet.get_destination(),
                                        tcp_packet.get_destination()
                                    );

                                    let payload = tcp_packet.payload();
                                    let text = String::from_utf8_lossy(payload);
                                    println!("TCP Payload (lossy):\n{}", text);
                                }
                            }
                            IpNextHeaderProtocols::Udp => {
                                println!("UDP packet (not parsed yet)");
                            }
                            other => {
                                println!("Other protocol: {:?}", other);
                            }
                        }
                    }
                }
                _ => {
                    println!("Unknown or unsupported ethertype.");
                }
            }
        }
    }
    Err(e) => eprintln!("Error reading packet: {}", e),
}
    }
}
