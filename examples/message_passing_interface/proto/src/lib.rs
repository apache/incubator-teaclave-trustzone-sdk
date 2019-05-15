use serde::{Serialize, Deserialize};
pub use serde_json;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Command {
    Hello,
    Bye,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnclaveInput {
    pub command: Command,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnclaveOutput {
    pub message: String
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Hello,
            1 => Command::Bye,
            _ => Command::Unknown,
        }
    }
}


pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
