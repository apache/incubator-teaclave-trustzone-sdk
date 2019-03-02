use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    cfg.target("aarch64-unknown-linux-gnu")
        .header("tee_api_types.h")
        .header("tee_api_defines.h")
        .header("utee_types.h")
        .header("user_ta_header.h")
        .header("tee_api.h")
        .include(env::var("OPTEE_OS_INCLUDE").unwrap())
        .type_name(|s, _is_struct, _is_union| {
            if s == "utee_params"
                || s == "ta_head"
                || s == "utee_attribute"
                || s == "user_ta_property"
            {
                return format!("struct {}", s);
            }
            s.to_string()
        });
    cfg.skip_struct(|s| {
        s == "Memref"
            || s == "Value"
            || s == "content"
            || s.ends_with("Handle")
            || s == "ta_prop"
            || s == "user_ta_property"
    });
    cfg.skip_field(|s, field| {
        (s == "ta_head" && field == "entry")
            || field == "content"
            || field == "value"
            || field == "memref"
            || field == "keyInformation"
    });
    cfg.skip_type(|s| s == "Memref" || s == "Value");
    cfg.skip_fn(|s| s == "TEE_BigIntFMMConvertToBigInt");
    cfg.skip_const(|s| s.starts_with("TA_PROP_STR") || s == "TEE_HANDLE_NULL");
    cfg.generate("../optee-utee-sys/src/lib.rs", "all.rs");
    println!("cargo:rustc-link-lib=static=mpa");
    println!("cargo:rustc-link-lib=static=utee");
    println!("cargo:rustc-link-lib=static=utils");

    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("aarch64-linux-gnu-gcc")
        .args(&["src/undefined.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/undefined.o", out_dir))
        .status()
        .unwrap();
    Command::new("aarch64-linux-gnu-ar")
        .args(&["crus", "libundefined.a", "undefined.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=undefined");
}
