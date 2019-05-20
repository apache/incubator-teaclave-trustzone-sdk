pub enum Command {
    Test,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Test,
            _ => Command::Unknown,
        }
    }
}

pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
