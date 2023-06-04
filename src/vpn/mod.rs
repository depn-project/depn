use core::fmt;
use std::ops::Deref;

mod openvpn;
mod wireguard;

pub use openvpn::OpenVPN;
use prettytable::{row, table};
pub use wireguard::WireGuard;

pub struct Connection {
    ip: std::net::Ipv4Addr,
}

pub struct ProtocolInfo {
    pub id: usize,
    pub name: String,
}

impl fmt::Display for ProtocolInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {}\nName: {}", self.id, self.name)
    }
}

pub trait Protocol {
    fn get_info(&self) -> ProtocolInfo;
}

pub fn print_protocols_table(protocols: Vec<ProtocolInfo>) {
    let mut table = table!(["ID", "Name"]);

    protocols.iter().for_each(|p| {
        table.add_row(row![p.id, p.name]);
    });

    table.printstd();
}

pub struct Manager<'a> {
    connections: Vec<Connection>,
    protocols: Vec<Box<&'a dyn Protocol>>,
}

impl<'a> Manager<'a> {
    pub fn new() -> Self {
        Manager {
            connections: vec![],
            protocols: vec![],
        }
    }

    pub fn protocol(mut self, vpn: &'a dyn Protocol) -> Self {
        self.protocols.push(Box::new(vpn));

        self
    }

    pub fn available_protocols(&self) -> Vec<ProtocolInfo> {
        self.protocols
            .iter()
            .map(|protocol| protocol.deref().get_info())
            .collect()
    }
}
