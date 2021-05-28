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
    let optee_client_dir = env::var("OPTEE_CLIENT_DIR").unwrap_or("../../optee/optee_client".to_string());
    let search_path = Path::new(&optee_client_dir).join("out/export/usr/lib");
    println!("cargo:rustc-link-search={}", search_path.display());
    println!("cargo:rustc-link-lib=static=teec");
}
