mod parser;
mod sniffer;
mod filter;
mod formatter;


use std::net::IpAddr;
use formatter::formatter::PacketFormatter;

use clap::Parser;

use crate::{ filter::filter::{ PacketFilter, Protocol }, formatter::formatter::OutputFormat, parser::ParsedTransport };

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

    #[arg[long = "format", value_enum]]
    format: Option<OutputFormat>

}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let packet_filter = PacketFilter::from_args(&args);

    let callback = Box::new(move |network_packet: parser::ParsedPacket| {
        if packet_filter.matches(network_packet.clone()) {

            match args.format {
                Some(OutputFormat::Json) => {
                    println!("{}", formatter::json::Json::print(network_packet));
                },
                Some(OutputFormat::Compact) => {
                    println!("{}", formatter::compact::Compact::print(network_packet));
                },
                _ => {}
            }
        }
    });
    sniffer::sniff(args.interface, callback);
    Ok(())
}
