pub enum Command {
    Prepare,
    SetKey,
    SetIV,
    Cipher,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Prepare,
            1 => Command::SetKey,
            2 => Command::SetIV,
            3 => Command::Cipher,
            _ => Command::Unknown,
        }
    }
}

pub enum Algo {
    ECB,
    CBC,
    CTR,
    Unknown,
}

impl From<u32> for Algo {
    #[inline]
    fn from(value: u32) -> Algo {
        match value {
            0 => Algo::ECB,
            1 => Algo::CBC,
            2 => Algo::CTR,
            _ => Algo::Unknown,
        }
    }
}
pub enum Mode {
    Decode,
    Encode,
    Unknown,
}

impl From<u32> for Mode {
    #[inline]
    fn from(value: u32) -> Mode {
        match value {
            0 => Mode::Decode,
            1 => Mode::Encode,
            _ => Mode::Unknown,
        }
    }
}

pub enum KeySize {
    Bit128 = 16,
    Bit256 = 32,
    Unknown = 0,
}

impl From<u32> for KeySize {
    #[inline]
    fn from(value: u32) -> KeySize {
        match value {
            16 => KeySize::Bit128,
            32 => KeySize::Bit256,
            _ => KeySize::Unknown,
        }
    }
}

pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
