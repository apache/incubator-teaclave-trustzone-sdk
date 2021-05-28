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
