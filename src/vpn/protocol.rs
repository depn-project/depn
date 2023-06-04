use core::fmt;

pub struct ProtocolInfo {
    pub id: usize,
    pub name: String,
}

pub struct ProtocolConfig;

impl fmt::Display for ProtocolInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {}\nName: {}", self.id, self.name)
    }
}

pub trait Protocol {
    fn get_info(&self) -> ProtocolInfo;
    fn create_config(&self) -> ProtocolConfig;
    fn remove_config(&self);
}
