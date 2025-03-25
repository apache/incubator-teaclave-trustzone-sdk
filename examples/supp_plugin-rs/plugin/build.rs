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
use std::path::PathBuf;
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut buffer = File::create(out.join("plugin_static.rs"))?;
    buffer.write_all(include_bytes!("plugin_static.rs"))?;

    let plugin_uuid = Uuid::parse_str(proto::PLUGIN_UUID).unwrap();
    let (time_low, time_mid, time_hi_and_version, clock_seq_and_node) = plugin_uuid.as_fields();

    write!(buffer, "\n")?;
    write!(
        buffer,
        "const PLUGIN_UUID_STRUCT: optee_teec::raw::TEEC_UUID = optee_teec::raw::TEEC_UUID {{
    timeLow: {:#x},
    timeMid: {:#x},
    timeHiAndVersion: {:#x},
    clockSeqAndNode: {:#x?},
}};",
        time_low, time_mid, time_hi_and_version, clock_seq_and_node
    )?;

    Ok(())
}
