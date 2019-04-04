use std::env;
use std::path::Path;

fn main() {
    let optee_client_dir = env::var("OPTEE_CLIENT_DIR").unwrap_or("../../optee/optee_client".to_string());
    let search_path = Path::new(&optee_client_dir).join("out/export/usr/lib");
    println!("cargo:rustc-link-search={}", search_path.display());
    println!("cargo:rustc-link-lib=static=teec");
}
