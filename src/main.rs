mod network;
use std::f32::consts::E;
use std::fmt::Debug;
use std::str::from_utf8;

use clap::Parser;
use network::{ Network, NetworkPacket };

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

fn main() {
    let args = Args::parse();
    println!("interface {}!", args.interface);
    println!("port {}!", args.port);
    let network = Network::new();
    let filter = args.filter.unwrap().clone();
    let callback = Box::new(move |network_packet: Box<dyn NetworkPacket>| {
        println!("{}", network_packet.protocol())
        // if network_packet.protocol.iter().len() > 0 && network_packet.protocol == Some(filter.clone()) {
        //     println!("{:?}", network_packet);
        // }
    });
    network.sniff(args.interface, callback)
}
