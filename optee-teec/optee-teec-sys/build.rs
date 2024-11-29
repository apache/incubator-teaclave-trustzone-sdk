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
use std::path::Path;

fn main() {
    const ENV_SYS_BUILD_TYPE: &str = "SYS_BUILD_TYPE";
    println!("cargo:rerun-if-env-changed={}", ENV_SYS_BUILD_TYPE);

    let build_type = env::var(ENV_SYS_BUILD_TYPE).unwrap_or(String::from("")).to_lowercase();
    match build_type.as_str() {
        "unit_test" => unit_test_build(),
        _ => production_build(),
    }
}

// this allow developers to run unit tests in host machine(even x86)
fn unit_test_build() {
}

fn production_build() {
    let optee_client_dir = env::var("OPTEE_CLIENT_EXPORT").expect("OPTEE_CLIENT_EXPORT is not set");
    let search_path = Path::new(&optee_client_dir).join("usr/lib");
    println!("cargo:rustc-link-search={}", search_path.display());
    println!("cargo:rustc-link-lib=dylib=teec");
}
