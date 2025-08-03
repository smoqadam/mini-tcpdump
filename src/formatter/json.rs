use crate::formatter::formatter::PacketFormatter;
use crate::parser::ParsedPacket;

pub struct Json;

impl PacketFormatter for Json {
    fn print(packet: ParsedPacket) -> String {
        serde_json::to_string(&packet).unwrap_or_else(|_| "{}".to_string())
    }
}