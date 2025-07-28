use std::f32::consts::E;
use std::fmt::Debug;
use std::str::from_utf8;

use clap::Parser;

use pnet::datalink::{ self, NetworkInterface };
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::{ Packet, MutablePacket };
use pnet::packet::ethernet::{ EtherTypes, EthernetPacket, MutableEthernetPacket };

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    interface: String,

    #[arg(short, long)]
    filter: String, //for filtering protocols tcp, http, etc. todo: use enum later

    #[arg(short, long)]
    port: u16,
}

#[derive(Debug)]
struct Request {
    src_mac: Option<String>,
    dest_mac: Option<String>,
    src: Option<String>,
    dest: Option<String>,
    protocol: Option<String>,
    flags: Option<u8>,
}

fn main() {
    let args = Args::parse();
    println!("interface {}!", args.interface);
    println!("filter {}!", args.filter);
    println!("port {}!", args.port);
    let network = Network::new();
    let callback = Box::new(|req: Request| {
        println!("{:?}", req);
    });
    network.sniff(args.interface, callback)
}

struct Network {}
impl Network {
    fn new() -> Network {
        return Network {};
    }

    fn sniff(&self, interface: String, callback: Box<dyn Fn(Request)>) {
        let interface_name = interface;
        let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

        // Find the network interface with the provided name
        let interfaces = datalink::interfaces();
        let interface = interfaces.into_iter().filter(interface_names_match).next().unwrap();

        // Create a new channel, dealing with layer 2 packets
        let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
        };

        loop {
            match rx.next() {
                Ok(packet) => {
                    let mut request = Request { flags: None, dest: None, src: None, protocol: None, src_mac: None, dest_mac: None };
                    if let Some(eth_packet) = EthernetPacket::new(packet) {

                        request.dest_mac = Some(format!("{:?}", eth_packet.get_destination()));
                        request.src_mac = Some(format!("{:?}", eth_packet.get_source()));

                        match eth_packet.get_ethertype() {
                            EtherTypes::Ipv6 => {}
                            EtherTypes::Ipv4 => {
                                if let Some(ip_packet) = Ipv4Packet::new(eth_packet.payload()) {
                                    request.dest = Some(format!("{:?}", ip_packet.get_destination()));
                                    request.src = Some(format!("{:?}", ip_packet.get_source()));
                                    request.flags = Some(ip_packet.get_flags());

                                    match ip_packet.get_next_level_protocol() {
                                        IpNextHeaderProtocols::Tcp => {
                                            if let Some(tcp_packet) = TcpPacket::new(ip_packet.payload())
                                            {
                                                request.protocol = Some("tcp".to_string());
                                                // let payload = tcp_packet.payload();
                                                // let text = String::from_utf8_lossy(payload);
                                                // println!("TCP Payload (lossy):\n{}", text);
                                            }
                                        }
                                        IpNextHeaderProtocols::Udp => {
                                            request.protocol = Some("udp".to_string());

                                        }
                                        other => {
                                            request.protocol = Some(other.to_string());
                                        }
                                    }
                                }
                            }
                            _ => {
                                println!("Unknown or unsupported ethertype.");
                            }
                        }
                    }

                    callback(request);
                }
                Err(e) => eprintln!("Error reading packet: {}", e),
            }
        }
    
    }
}
