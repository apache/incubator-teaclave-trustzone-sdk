# Reference Implementation Examples for Web3 Trusted Application

Teaclave TrustZone SDK enables developers to write Trusted Applications (TAs) in Rust, offering a memory-safe environment for secure application development. Many of the examples in this repo are ported from OP-TEE C examples. However, thanks to Rust's robust crate ecosystem and the support for Rust-std in Teaclave TrustZone SDK, developers can now create more functional and secure TAs. These applications are designed to protect confidential information while maintaining Rust's inherent memory safety.

A particularly relevant scenario is in Web3, where private key protection is critical for securing on-chain identities and assets. In this context, TAs can protect the entire lifecycle of Web3 credentials, such as those used in self-custody wallets or validator signing key protection. In DePIN, TAs provide security for device attestation to mitigate Sybil device threats. This, for instance, allows safe airdropping to trusted devices, ensuring only verified devices are eligible.

This directory contains a collection of reference implementations of TAs, specifically tailored for Web3 use cases. These examples demonstrate how to use Rust within TrustZone to support basic Web3 use cases. We will gradually open-source each of them as reference implementation example as Web3 TA. Web3 builders can leverage these examples to integrate secure functionalities into their projects, particularly in environments where OP-TEE and TrustZone technologies are employed.

## Basic Web3 Wallet
AVAILABLE in [eth-wallet/](./eth-wallet)

A wallet abstraction featuring key functionalities like secure key management and transaction signing. The key management includes secure seed generation, mnemonic derivation, and safe key storage within external TEE-protected environments. For transaction signing, we demonstrate how to securely sign an Ethereum transaction using wallet-derived keys inside the TEE, ensuring the private keys never leave the trusted environment.

## X509 Certificate Signing & Verification
Status: To Be Released

Basic Public Key Infrastructure (PKI) primitives that securely issue self-signed certificates and verify externally provided leaf certificates through a trusted certificate store. The Trusted Application (TA) running inside the TEE is responsible for securely generating key pairs and issuing self-signed certificates, used for identity verification in secure communications. It also handles the verification of certificates provided by external entities, ensuring their validity by cross-referencing them with a secure certificate store within the TEE. This primitive is particularly useful for establishing trusted communication channels between nodes, validators, or devices in Web3 environments, where trust and identity verification are critical for decentralized interactions.

## Remote Attestation
Status: To Be Released

A foundational primitive used to remotely attest the authenticity of a Trusted Application (TA), ensuring it is indeed running in a Trusted Execution Environment (TEE). This primitive leverages TLS and X509 PKI to establish a secure communication channel. During the TLS handshake, the TA generates an attestation report, which can be signed using its private key within the TEE. The remote party then verifies this report, confirming the TA’s integrity and the secure execution environment. In Web3 scenarios, remote attestation ensures that only trusted devices and applications can participate in sensitive operations, such as airdrop or validator duties. This guarantees the authenticity and trustworthiness of participants in decentralized systems, mitigating risks like Sybil attacks and unauthorized access.

## Multi-Factor Authentication (MFA)
Status: To Be Released

In Web3, ensuring explicit user confirmation through a trusted channel is a challenging task. This example demonstrates how to implement MFA mechanisms by securely provisioning the public keys of trusted MFA devices (e.g., the user’s cellphone) within the Trusted Application (TA). When a high-risk operation, such as key usage or transaction signing, requires user confirmation, the TA can verify the details confirmed by the user via the trusted MFA device, without relying on any third party. This empowers Web3 builders to create more secure, decentralized systems where user interactions are reliably authenticated for critical tasks like transaction signing or private key access.


