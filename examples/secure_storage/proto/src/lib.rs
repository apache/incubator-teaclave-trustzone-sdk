pub enum Command {
    Read,
    Write,
    Delete,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Read,
            1 => Command::Write,
            2 => Command::Delete,
            _ => Command::Unknown,
        }
    }
}

pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
