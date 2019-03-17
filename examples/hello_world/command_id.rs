pub enum Command {
    IncValue,
    DecValue,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::IncValue,
            1 => Command::DecValue,
            _ => Command::Unknown,
        }
    }
}
