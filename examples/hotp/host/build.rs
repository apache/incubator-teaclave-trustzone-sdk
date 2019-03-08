use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out.join("host_header.rs"))
        .unwrap()
        .write_all(include_bytes!("../command_id.rs"))
        .unwrap();
}
