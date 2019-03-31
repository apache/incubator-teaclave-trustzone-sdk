use std::env;
use std::path::Path;

fn main() {
    let optee_os_dir = env::var("OPTEE_OS_DIR").unwrap_or("../../optee/optee_os".to_string());
    let search_path = Path::new(&optee_os_dir).join("out/arm/export-ta_arm64/lib");
    println!("cargo:rustc-link-search={}", search_path.display());
    println!("cargo:rustc-link-lib=static=utee");
    println!("cargo:rustc-link-lib=static=mbedtls");
}
