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

// For CA-TA invocation:
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Command {
    Test,
    #[default]
    Unknown,
}

// If Uuid::parse_str() returns an InvalidLength error, there may be an extra
// newline in your uuid.txt file. You can remove it by running
// `truncate -s 36 uuid.txt`.
pub const UUID: &str = &include_str!("../../uuid.txt");

// For TA-TA invocation testcases:
#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum SystemPtaCommand {
    AddRngEntropy,
    DeriveTaUniqueKey,
    // We omit other commands here.
    // Full definitions can be found in optee_os system_pta.h.
    #[default]
    Unknown,
}

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum HelloWorldTaCommand {
    IncValue,
    DecValue,
    #[default]
    Unknown,
}

pub const SYSTEM_PTA_UUID: &str = "3a2f8978-5dc0-11e8-9c2d-fa7ae01bbebc";
pub const HELLO_WORLD_USER_TA_UUID: &str = "133af0ca-bdab-11eb-9130-43bf7873bf67";
