pub enum Command {
    Prepare,
    Update,
    EncFinal,
    DecFinal,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Prepare,
            1 => Command::Update,
            2 => Command::EncFinal,
            3 => Command::DecFinal,
            _ => Command::Unknown,
        }
    }
}

pub enum Mode {
    Encrypt,
    Decrypt,
    Unknown,
}

impl From<u32> for Mode {
    #[inline]
    fn from(value: u32) -> Mode {
        match value {
            0 => Mode::Encrypt,
            1 => Mode::Decrypt,
            _ => Mode::Unknown,
        }
    }
}

pub const BUFFER_SIZE: usize = 16;
pub const KEY_SIZE: usize = 16;
pub const AAD_LEN: usize = 16;
pub const TAG_LEN: usize = 16;

pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
