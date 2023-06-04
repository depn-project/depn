use super::{Protocol, ProtocolInfo};

pub struct WireGuard {
    info: ProtocolInfo,
}

impl WireGuard {
    pub fn new() -> Self {
        Self {
            info: ProtocolInfo {
                id: 1,
                name: "WireGuard".to_string(),
            },
        }
    }
}

impl Protocol for WireGuard {
    fn get_info(&self) -> ProtocolInfo {
        ProtocolInfo {
            name: self.info.name.clone(),
            ..self.info
        }
    }
}
