use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut m_file = File::create(out.join("user_ta_header.rs")).unwrap();
    m_file.write_all(include_bytes!("ta_static.rs")).unwrap();

    m_file.write(b"\n").unwrap();
    m_file
        .write_all(include_bytes!("../command_id.rs"))
        .unwrap();

    let tee_uuid = Uuid::parse_str(&include_str!("../uuid.txt").trim()).unwrap();
    let (time_low, time_mid, time_hi_and_version, clock_seq_and_node) = tee_uuid.as_fields();

    m_file.write(b"\n").unwrap();
    m_file
        .write_fmt(format_args!(
            "const TA_UUID: TEE_UUID = TEE_UUID {{
    timeLow: {:#x},
    timeMid: {:#x},
    timeHiAndVersion: {:#x},
    clockSeqAndNode: {:#x?},
}};",
            time_low, time_mid, time_hi_and_version, clock_seq_and_node
        ))
        .unwrap();

    File::create(out.join("ta.lds"))
        .unwrap()
        .write_all(include_bytes!("ta.lds"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=ta.lds");

    let optee_os_dir = env::var("OPTEE_OS_DIR").unwrap_or("../../../optee/optee_os".to_string());
    let search_path = Path::new(&optee_os_dir).join("out/arm/export-ta_arm64/lib");
    println!("cargo:rustc-link-search={}", search_path.display());
    println!("cargo:rustc-link-lib=static=mpa");
}
