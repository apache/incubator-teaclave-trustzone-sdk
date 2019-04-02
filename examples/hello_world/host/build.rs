use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{PathBuf, Path};
use uuid::Uuid;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out.join("host_header.rs"))
        .unwrap()
        .write_all(include_bytes!("../command_id.rs"))
        .unwrap();

    if Path::new("../uuid.txt").exists() == false {
        let uuid = Uuid::new_v4();
        fs::write("../uuid.txt", uuid.to_string()).unwrap();
    }
}
