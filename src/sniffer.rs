use std::io::{Error, ErrorKind};
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;

use crate::parser;
use crate::parser::ParsedPacket;

pub fn sniff(interface: String, callback: Box<dyn Fn(ParsedPacket)>) -> Result<(), Error> {
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface;

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(interface_names_match)
        .ok_or_else(|| Error::new(ErrorKind::NotFound, format!("Interface '{}' not found", interface)))?;

    let mut rx = match datalink::channel(&interface, Default::default())? {
        Ethernet(_, rx) => rx,
        _ => return Err(Error::new(
            ErrorKind::Unsupported, 
            "Only Ethernet interfaces are supported"
        )),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(p) = parser::parser::parse(packet) {
                    callback(p);
                }
            }
            Err(e) => eprintln!("Error reading packet: {}", e),
        }
    }
}
