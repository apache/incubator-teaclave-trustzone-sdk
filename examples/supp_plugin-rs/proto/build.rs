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

use std::fs;
use std::path::PathBuf;
use std::fs::File;
use std::env;
use std::io::Write;

fn main() {
    //ta uuid
    let uuid = match fs::read_to_string("../ta_uuid.txt") {
        Ok(u) => {
            u.trim().to_string()
        },
        Err(_) => {
            panic!("Cannot find ta_uuid.txt");
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
            panic!("Cannot find plugin_uuid.txt");
        }
    };
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut buffer = File::create(out.join("plugin_uuid.txt")).unwrap();
    write!(buffer, "{}", uuid).unwrap();
}
