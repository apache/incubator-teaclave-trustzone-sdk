// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    cfg.target("aarch64-unknown-linux-gnu")
        .header("tee_api_types.h")
        .header("tee_api_defines.h")
        .header("utee_types.h")
        .header("user_ta_header.h")
        .header("tee_api.h")
        .header("utee_syscalls.h")
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
    cfg.skip_roundtrip(|s| s.starts_with("TEE_") || s.starts_with("utee_") || s == "ta_head");
    cfg.generate("../optee-utee-sys/src/lib.rs", "all.rs");
    println!("cargo:rustc-link-lib=static=mbedtls");
    println!("cargo:rustc-link-lib=static=utee");
    println!("cargo:rustc-link-lib=static=utils");

    let out_dir = env::var("OUT_DIR").unwrap();
    let undefined_path = PathBuf::from(&out_dir).join("undefined.c");

    let mut buffer = File::create(&undefined_path).unwrap();
    write!(
        buffer,
        "
        void* ta_props = 0;
        void* ta_num_props = 0;
        void* trace_level = 0;
        void* trace_ext_prefix = 0;
    "
    )
    .unwrap();
    Command::new("aarch64-linux-gnu-gcc")
        .args(&[undefined_path.to_str().unwrap(), "-c", "-fPIC", "-o"])
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
