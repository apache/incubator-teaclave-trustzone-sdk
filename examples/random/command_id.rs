pub enum Command {
    RandomGenerator,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::RandomGenerator,
            _ => Command::Unknown,
        }
    }
}
