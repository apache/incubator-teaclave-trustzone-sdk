use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out.join("user_ta_header.rs"))
        .unwrap()
        .write_all(include_bytes!("ta_static.rs"))
        .unwrap();

    let mut m_file = OpenOptions::new()
        .append(true)
        .open(out.join("user_ta_header.rs"))
        .unwrap();
    let _ = writeln!(m_file, "");
    let _ = m_file.write_all(include_bytes!("../command_id.rs"));

    let teeuuid = Uuid::parse_str(&include_str!("../uuid.txt").trim()).unwrap();
    let _ = writeln!(m_file, "");
    let _ = writeln!(m_file, "const TA_UUID: TEE_UUID = TEE_UUID {{");
    let _ = writeln!(m_file, "   timeLow: {:#x},", teeuuid.as_fields().0);
    let _ = writeln!(m_file, "   timeMid: {:#x},", teeuuid.as_fields().1);
    let _ = writeln!(m_file, "   timeHiAndVersion: {:#x},", teeuuid.as_fields().2);
    let _ = writeln!(m_file, "   clockSeqAndNode: {:#x?},", teeuuid.as_fields().3);
    let _ = writeln!(m_file, "}};");

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
