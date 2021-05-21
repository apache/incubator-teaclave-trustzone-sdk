//for ta
pub enum Command {
    Ping,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Ping,
            _ => Command::Unknown,
        }
    }
}

pub const TA_UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/ta_uuid.txt"));

//for plugin
pub enum PluginCommand {
    Print,
    Unknown,
}

impl From<u32> for PluginCommand {
    #[inline]
    fn from(value: u32) -> PluginCommand {
        match value {
            0 => PluginCommand::Print,
            _ => PluginCommand::Unknown,
        }
    }
}

pub const PLUGIN_SUBCMD_NULL: u32 = 0xFFFFFFFF;
pub const PLUGIN_UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/plugin_uuid.txt"));

