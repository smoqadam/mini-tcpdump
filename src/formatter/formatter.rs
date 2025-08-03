use crate::parser::ParsedPacket;


#[derive(Clone, Debug, clap::ValueEnum)]
pub enum OutputFormat {
    Json, 
    Compact
}
pub trait PacketFormatter {
    fn print(packet: ParsedPacket) -> String;
}