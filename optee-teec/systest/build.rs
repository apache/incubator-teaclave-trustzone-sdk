use std::env;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    cfg.target("aarch64-unknown-linux-gnu")
       .header("tee_client_api.h")
       .include(env::var("OPTEE_CLIENT_INCLUDE").unwrap())
       .type_name(|s, _is_struct, _is_union| {
            s.to_string()
    });
    cfg.generate("../optee-teec-sys/src/lib.rs", "all.rs");
}
