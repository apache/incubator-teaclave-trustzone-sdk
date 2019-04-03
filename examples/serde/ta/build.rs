use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut buffer = File::create(out.join("user_ta_header.rs"))?;
    buffer.write_all(include_bytes!("ta_static.rs"))?;
    write!(buffer, "\n")?;
    buffer.write_all(include_bytes!("../command_id.rs"))?;

    let tee_uuid = match Path::new("../uuid.txt").exists() {
        true => Uuid::parse_str(&fs::read_to_string("../uuid.txt").unwrap().trim()).unwrap(),
        false => {
            let uuid = Uuid::new_v4();
            fs::write("../uuid.txt", uuid.to_string())?;
            uuid
        }
    };
    let (time_low, time_mid, time_hi_and_version, clock_seq_and_node) = tee_uuid.as_fields();

    write!(buffer, "\n")?;
    write!(
        buffer,
        "const TA_UUID: optee_utee_sys::TEE_UUID = optee_utee_sys::TEE_UUID {{
    timeLow: {:#x},
    timeMid: {:#x},
    timeHiAndVersion: {:#x},
    clockSeqAndNode: {:#x?},
}};",
        time_low, time_mid, time_hi_and_version, clock_seq_and_node
    )?;

    File::create(out.join("ta.lds"))?.write_all(include_bytes!("ta.lds"))?;
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=ta.lds");

    let optee_os_dir = env::var("OPTEE_OS_DIR").unwrap_or("../../../optee/optee_os".to_string());
    let search_path = Path::new(&optee_os_dir).join("out/arm/export-ta_arm64/lib");
    println!("cargo:rustc-link-search={}", search_path.display());
    println!("cargo:rustc-link-lib=static=utee");
    Ok(())
}
