use proto;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut buffer = File::create(out.join("plugin_static.rs"))?;
    buffer.write_all(include_bytes!("plugin_static.rs"))?;

    let plugin_uuid = Uuid::parse_str(proto::PLUGIN_UUID).unwrap();
    let (time_low, time_mid, time_hi_and_version, clock_seq_and_node) = plugin_uuid.as_fields();

    write!(buffer, "\n")?;
    write!(
        buffer,
        "const PLUGIN_UUID_STRUCT: optee_teec_sys::TEEC_UUID = optee_teec_sys::TEEC_UUID {{
    timeLow: {:#x},
    timeMid: {:#x},
    timeHiAndVersion: {:#x},
    clockSeqAndNode: {:#x?},
}};",
        time_low, time_mid, time_hi_and_version, clock_seq_and_node
    )?;

    Ok(())
}
