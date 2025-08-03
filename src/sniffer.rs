use pnet::datalink::{ self, NetworkInterface };
use pnet::datalink::Channel::Ethernet;

use crate::parser;
use crate::parser::ParsedPacket;

pub fn sniff(interface: String, callback: Box<dyn Fn(ParsedPacket)>) {
    let interface_name = interface;
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().filter(interface_names_match).next().unwrap();

    let mut rx = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        Ok(_) => unimplemented!(),
        Err(e) => {
            unimplemented!();
        }
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
