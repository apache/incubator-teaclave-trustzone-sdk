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

use serde::{Serialize, Deserialize};
pub use serde_json;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Command {
    Hello,
    Bye,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnclaveInput {
    pub command: Command,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnclaveOutput {
    pub message: String
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Hello,
            1 => Command::Bye,
            _ => Command::Unknown,
        }
    }
}


pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
