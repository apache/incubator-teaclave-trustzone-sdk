pub enum Command {
    DefaultOp,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::DefaultOp,
            _ => Command::Unknown,
        }
    }
}
