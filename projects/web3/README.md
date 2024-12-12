# Reference Implementation Examples for Web3 Trusted Applications

Teaclave TrustZone SDK allows developers to create Trusted Applications (TAs) in
Rust, offering a memory-safe and secure environment. Many examples in this
repository are ported from OP-TEE C examples. With Rust's ecosystem and support
for Rust-std in Teaclave TrustZone SDK, developers can build secure TAs to
protect confidential information.

In Web3, private key protection is vital for securing on-chain identities and
assets. TAs safeguard the entire lifecycle of Web3 credentials used in wallets
or validator key protection. In DePIN, TAs enable secure device attestation,
helping to prevent Sybil attacks.

This directory contains a collection of reference implementations of TAs,
specifically tailored for Web3 use cases. These examples demonstrate how to use
Rust within TrustZone to support basic Web3 use cases. We will gradually
open-source each of them as reference implementation examples for Web3 TAs. Web3
builders can leverage these examples to integrate secure functionalities into
their projects, particularly in environments where OP-TEE and TrustZone
technologies are employed.

## Basic Web3 Wallet

**AVAILABLE** in [eth-wallet/](./eth-wallet)

A wallet abstraction featuring key functionalities like secure key management
and transaction signing. The key management includes secure seed generation,
mnemonic derivation, and safe key storage within external TEE-protected
environments. For transaction signing, we demonstrate how to securely sign an
Ethereum transaction using wallet-derived keys inside the TEE, ensuring the
private keys never leave the trusted environment.

## Decentralized Identifier (DID)

**To Be Released**

This example will illustrate how to integrate Decentralized Identifiers (DIDs)
into TAs. DIDs enable self-sovereign identity by proving ownership without
relying on central authorities. Secure key management for creating and operating
DIDs ensures reliable device identification, mitigating the risk of fake devices
in DePIN.
