pub enum Command {
    RegisterSharedKey,
    GetHOTP,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::RegisterSharedKey,
            1 => Command::GetHOTP,
            _ => Command::Unknown,
        }
    }
}
