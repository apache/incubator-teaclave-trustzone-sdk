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

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateWalletInput {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateWalletOutput {
    pub wallet_id: Uuid,
    pub mnemonic: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveWalletInput {
    pub wallet_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveWalletOutput {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeriveAddressInput {
    pub wallet_id: Uuid,
    pub hd_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeriveAddressOutput {
    pub address: [u8; 20],
    pub public_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EthTransaction {
    pub chain_id: u64,
    pub nonce: u128,
    pub to: Option<[u8; 20]>,
    pub value: u128,
    pub gas_price: u128,
    pub gas: u128,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignTransactionInput {
    pub wallet_id: Uuid,
    pub hd_path: String,
    pub transaction: EthTransaction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignTransactionOutput {
    pub signature: Vec<u8>,
}
