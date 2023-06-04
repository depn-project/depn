use std::ops::Deref;

mod protocol;

mod wireguard;

use prettytable::{row, table};
pub use protocol::{Protocol, ProtocolConfig, ProtocolInfo};
pub use wireguard::WireGuard;

pub struct Connection {
    ip: std::net::Ipv4Addr,
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
        Self {
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
