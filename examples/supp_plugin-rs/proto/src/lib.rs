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
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Command {
    Ping,
    #[default]
    Unknown,
}

// If Uuid::parse_str() returns an InvalidLength error, there may be an extra
// newline in your uuid.txt file. You can remove it by running
// `truncate -s 36 ta_uuid.txt`.
pub const TA_UUID: &str = &include_str!("../../ta_uuid.txt");

//for plugin
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum PluginCommand {
    Print,
    #[default]
    Unknown,
}

pub const PLUGIN_SUBCMD_NULL: u32 = 0xFFFFFFFF;
// If Uuid::parse_str() returns an InvalidLength error, there may be an extra
// newline in your uuid.txt file. You can remove it by running
// `truncate -s 36 plugin_uuid.txt`.
pub const PLUGIN_UUID: &str = &include_str!("../../plugin_uuid.txt");
