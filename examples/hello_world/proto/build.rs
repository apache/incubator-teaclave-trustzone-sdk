use std::fs;
use std::path::PathBuf;
use std::fs::File;
use uuid::Uuid;
use std::env;
use std::io::Write;

fn main() {
    let uuid = match fs::read_to_string("../uuid.txt") {
        Ok(u) => {
            u.trim().to_string()
        },
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
