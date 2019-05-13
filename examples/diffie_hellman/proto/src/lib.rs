pub enum Command {
    GenerateKey,
    DeriveKey,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::GenerateKey,
            1 => Command::DeriveKey,
            _ => Command::Unknown,
        }
    }
}

pub const KEY_SIZE: usize = 256;

pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
