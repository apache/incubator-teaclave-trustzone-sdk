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

use anyhow::{anyhow, Result};
use bip32::{Mnemonic, XPrv};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

use crate::hash::keccak_hash_to_bytes;
use ethereum_tx_sign::Transaction;
use optee_utee::Random;
use proto::EthTransaction;
use secure_db::Storable;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Wallet {
    id: Uuid,
    entropy: Vec<u8>,
}

impl Storable for Wallet {
    type Key = Uuid;

    fn unique_id(&self) -> Self::Key {
        self.id
    }
}

impl Wallet {
    pub fn new() -> Result<Self> {
        let mut entropy = vec![0u8; 32];
        Random::generate(entropy.as_mut() as _);

        let mut random_bytes = vec![0u8; 16];
        Random::generate(random_bytes.as_mut() as _);
        let uuid = uuid::Builder::from_random_bytes(
            random_bytes
                .try_into()
                .map_err(|_| anyhow!("[-] Wallet::new(): invalid random bytes"))?,
        )
        .into_uuid();

        Ok(Self {
            id: uuid,
            entropy: entropy,
        })
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn get_mnemonic(&self) -> Result<String> {
        let mnemonic = Mnemonic::from_entropy(
            self.entropy.as_slice().try_into()?,
            bip32::Language::English,
        );
        Ok(mnemonic.phrase().to_string())
    }

    pub fn get_seed(&self) -> Result<Vec<u8>> {
        let mnemonic = Mnemonic::from_entropy(
            self.entropy.as_slice().try_into()?,
            bip32::Language::English,
        );
        let seed = mnemonic.to_seed(""); // empty passwords
        Ok(seed.as_bytes().to_vec())
    }

    pub fn derive_prv_key(&self, hd_path: &str) -> Result<Vec<u8>> {
        let path = hd_path.parse()?;
        let child_xprv = XPrv::derive_from_path(self.get_seed()?, &path)?;
        let child_xprv_bytes = child_xprv.to_bytes();
        Ok(child_xprv_bytes.to_vec())
    }

    pub fn derive_pub_key(&self, hd_path: &str) -> Result<Vec<u8>> {
        let path = hd_path.parse()?;
        let child_xprv = XPrv::derive_from_path(self.get_seed()?, &path)?;
        // public key
        let child_xpub_bytes = child_xprv.public_key().to_bytes();
        Ok(child_xpub_bytes.to_vec())
    }

    pub fn derive_address(&self, hd_path: &str) -> Result<([u8; 20], Vec<u8>)> {
        let public_key_bytes = self.derive_pub_key(hd_path)?;
        // uncompress public key
        let public_key = secp256k1::PublicKey::from_slice(&public_key_bytes)?;
        let uncompressed_public_key = &public_key.serialize_uncompressed()[1..];

        // pubkey to address
        let address = &keccak_hash_to_bytes(&uncompressed_public_key)[12..];
        Ok((address.try_into()?, public_key_bytes))
    }

    pub fn sign_transaction(&self, hd_path: &str, transaction: &EthTransaction) -> Result<Vec<u8>> {
        let xprv = self.derive_prv_key(hd_path)?;
        let legacy_transaction = ethereum_tx_sign::LegacyTransaction {
            chain: transaction.chain_id,
            nonce: transaction.nonce,
            gas_price: transaction.gas_price,
            gas: transaction.gas,
            to: transaction.to,
            value: transaction.value,
            data: transaction.data.clone(),
        };
        let ecdsa = legacy_transaction.ecdsa(&xprv).map_err(|e| {
            let ethereum_tx_sign::Error::Secp256k1(inner_error) = e;
            inner_error
        })?;
        let signature = legacy_transaction.sign(&ecdsa);
        Ok(signature)
    }
}

impl TryFrom<Wallet> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(wallet: Wallet) -> Result<Vec<u8>> {
        bincode::serialize(&wallet).map_err(|e| anyhow!("[-] Wallet::try_into(): {:?}", e))
    }
}

impl TryFrom<Vec<u8>> for Wallet {
    type Error = anyhow::Error;

    fn try_from(data: Vec<u8>) -> Result<Wallet> {
        bincode::deserialize(&data).map_err(|e| anyhow!("[-] Wallet::try_from(): {:?}", e))
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        self.entropy.iter_mut().for_each(|x| *x = 0);
    }
}
