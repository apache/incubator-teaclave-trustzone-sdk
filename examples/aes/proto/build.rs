use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

fn main() {
    let uuid = match fs::read_to_string("../uuid.txt") {
        Ok(u) => u.trim().to_string(),
        Err(_) => {
            let u = Uuid::new_v4().to_string();
            fs::write("../uuid.txt", &u).unwrap();
            u
        }
    };
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut buffer = File::create(out.join("uuid.txt")).unwrap();
    write!(buffer, "{}", uuid).unwrap();
}
