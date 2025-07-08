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

use optee_utee_sys as raw;

/// Represent the characteristics of an object.
/// This info can be returned by [TransientObject](TransientObject) function
/// [info](TransientObject::info)
/// or [PersistentObject](PersistentObject) function
/// [info](PersistentObject::info).
pub struct ObjectInfo {
    pub(crate) raw: raw::TEE_ObjectInfo,
}

// Since raw struct is not implemented Copy attribute yet, every item in raw struct needs a function to extract.
impl ObjectInfo {
    /// Return an [ObjectInfo](ObjectInfo) struct based on the raw structure `TEE_ObjectInfo`.
    /// The raw structure contains following fields:
    ///
    /// 1) `objectType`: The parameter represents one of the
    ///    [TransientObjectType](TransientObjectType).
    /// 2) `objectSize`: The current size in bits of the object as determined by its attributes.
    /// This will always be less than or equal to maxObjectSize. Set to 0 for uninitialized and data only objects.
    /// 3) `maxObjectSize`: The maximum objectSize which this object can represent.
    /// 3.1) For a [PersistentObject](PersistentObject), set to `objectSize`.
    /// 3.2) For a [TransientObject](TransientObject), set to the parameter `maxObjectSize` passed to
    /// [allocate](TransientObject::allocate).
    /// 4) `objectUsage`: A bit vector of UsageFlag.
    /// 5) `dataSize`:
    /// 5.1) For a [PersistentObject](PersistentObject), set to the current size of the data associated with the object.
    /// 5.2) For a [TransientObject](TransientObject), always set to 0.
    /// 6) `dataPosition`:
    /// 6.1) For a [PersistentObject](PersistentObject), set to the current position in the data for this handle.
    /// Data positions for different handles on the same object may differ.
    /// 6.2) For a [TransientObject](TransientObject), set to 0.
    /// 7) `handleFlags`: A bit vector containing one or more [HandleFlag](HandleFlag) or [DataFlag](DataFlag).
    pub fn from_raw(raw: raw::TEE_ObjectInfo) -> Self {
        Self { raw }
    }

    /// Return the `dataSize` field of the raw structure `TEE_ObjectInfo`.
    pub fn data_size(&self) -> usize {
        self.raw.dataSize as usize
    }

    /// Return the `objectSize` field of the raw structure `TEE_ObjectInfo`.
    pub fn object_size(&self) -> usize {
        self.raw.objectSize as usize
    }

    /// Return the `objectType` field of the raw structure `TEE_ObjectInfo`.
    pub fn object_type(&self) -> u32 {
        self.raw.objectType
    }
}
