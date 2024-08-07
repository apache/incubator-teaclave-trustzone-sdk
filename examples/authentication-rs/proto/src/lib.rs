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

#![no_std]

pub enum Command {
    Prepare,
    Update,
    EncFinal,
    DecFinal,
    Unknown,
}

impl From<u32> for Command {
    #[inline]
    fn from(value: u32) -> Command {
        match value {
            0 => Command::Prepare,
            1 => Command::Update,
            2 => Command::EncFinal,
            3 => Command::DecFinal,
            _ => Command::Unknown,
        }
    }
}

pub enum Mode {
    Encrypt,
    Decrypt,
    Unknown,
}

impl From<u32> for Mode {
    #[inline]
    fn from(value: u32) -> Mode {
        match value {
            0 => Mode::Encrypt,
            1 => Mode::Decrypt,
            _ => Mode::Unknown,
        }
    }
}

pub const BUFFER_SIZE: usize = 16;
pub const KEY_SIZE: usize = 16;
pub const AAD_LEN: usize = 16;
pub const TAG_LEN: usize = 16;

pub const UUID: &str = &include_str!(concat!(env!("OUT_DIR"), "/uuid.txt"));
