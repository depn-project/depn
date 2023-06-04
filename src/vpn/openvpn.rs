use super::{Protocol, ProtocolInfo};

pub struct OpenVPN {
    info: ProtocolInfo,
}

impl OpenVPN {
    pub fn new() -> Self {
        Self {
            info: ProtocolInfo {
                id: 2,
                name: "OpenVPN".to_string(),
            },
        }
    }
}

impl Protocol for OpenVPN {
    fn get_info(&self) -> ProtocolInfo {
        ProtocolInfo {
            name: self.info.name.clone(),
            ..self.info
        }
    }
}
