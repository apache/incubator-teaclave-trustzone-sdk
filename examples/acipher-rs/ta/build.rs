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

use proto;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut buffer = File::create(out.join("user_ta_header.rs"))?;
    buffer.write_all(include_bytes!("ta_static.rs"))?;

    let tee_uuid = Uuid::parse_str(proto::UUID).unwrap();
    let (time_low, time_mid, time_hi_and_version, clock_seq_and_node) = tee_uuid.as_fields();

    write!(buffer, "\n")?;
    write!(
        buffer,
        "const TA_UUID: optee_utee_sys::TEE_UUID = optee_utee_sys::TEE_UUID {{
    timeLow: {:#x},
    timeMid: {:#x},
    timeHiAndVersion: {:#x},
    clockSeqAndNode: {:#x?},
}};",
        time_low, time_mid, time_hi_and_version, clock_seq_and_node
    )?;

    let mut aarch64_flag = true;
    match env::var("TARGET") {
        Ok(ref v) if v == "arm-unknown-linux-gnueabihf" => {
            println!("cargo:rustc-link-arg=--no-warn-mismatch");
            aarch64_flag = false;
        },
        _ => {}
    };

    let optee_os_dir = env::var("TA_DEV_KIT_DIR").unwrap();
    let search_path = Path::new(&optee_os_dir).join("lib");

    let optee_os_path = &PathBuf::from(optee_os_dir.clone());
    let mut ta_lds = File::create(out.join("ta.lds"))?;
    let f = File::open(optee_os_path.join("src/ta.ld.S"))?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let l = line?;

        if aarch64_flag {
            if l.starts_with('#') ||
                l == "OUTPUT_FORMAT(\"elf32-littlearm\")" ||
                l == "OUTPUT_ARCH(arm)" {
                continue;
            }
        } else {
            if l.starts_with('#') ||
                l == "OUTPUT_FORMAT(\"elf64-littleaarch64\")" ||
                l == "OUTPUT_ARCH(aarch64)" {
                continue;
            }
        }

        if l == "\t. = ALIGN(4096);" {
            write!(ta_lds, "\t. = ALIGN(65536);\n")?;
        } else {
            write!(ta_lds, "{}\n", l)?;
        }
    }

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=ta.lds");

    println!("cargo:rustc-link-search={}", search_path.display());
    println!("cargo:rustc-link-lib=static=utee");
    println!("cargo:rustc-link-lib=static=utils");
    println!("cargo:rustc-link-arg=-Tta.lds");
    println!("cargo:rustc-link-arg=-e__ta_entry");
    println!("cargo:rustc-link-arg=-pie");
    println!("cargo:rustc-link-arg=-Os");
    //println!("cargo:rustc-link-arg=--sort-section=alignment");


    let mut dyn_list = File::create(out.join("dyn_list"))?;
    write!(dyn_list, "{{ __elf_phdr_info; trace_ext_prefix; trace_level; ta_head; }};\n")?;
    //println!("cargo:rustc-link-arg=--dynamic-list=dyn_list");
    Ok(())
}
