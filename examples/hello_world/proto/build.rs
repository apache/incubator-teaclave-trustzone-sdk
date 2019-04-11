use std::fs;
use std::path::Path;
use uuid::Uuid;

fn main() {
    if Path::new("../uuid.txt").exists() == false {
        let uuid = Uuid::new_v4();
        fs::write("../uuid.txt", uuid.to_string()).unwrap();
    }
}
