use std::fs;
use std::path::PathBuf;
use std::fs::File;
use uuid::Uuid;
use std::env;
use std::io::Write;

fn main() {
    //ta uuid
    let uuid = match fs::read_to_string("../ta_uuid.txt") {
        Ok(u) => {
            u.trim().to_string()
        },
        Err(_) => {
            let u = Uuid::new_v4().to_string();
            fs::write("../ta_uuid.txt", &u).unwrap();
            u
        }
    };
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut buffer = File::create(out.join("ta_uuid.txt")).unwrap();
    write!(buffer, "{}", uuid).unwrap();

    //plugin uuid
    let uuid = match fs::read_to_string("../plugin_uuid.txt") {
        Ok(u) => {
            u.trim().to_string()
        },
        Err(_) => {
            let u = Uuid::new_v4().to_string();
            fs::write("../plugin_uuid.txt", &u).unwrap();
            u
        }
    };
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut buffer = File::create(out.join("plugin_uuid.txt")).unwrap();
    write!(buffer, "{}", uuid).unwrap();
}
