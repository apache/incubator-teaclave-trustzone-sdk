# Projects in Multiple Scenarios

Trusted Execution Environments (TEEs) play a vital role in providing critical
security solutions across various scenarios. The Teaclave TrustZone SDK empowers
developers to implement robust use cases such as Web3 private key protection,
authentication, and more.

The `projects/` directory showcases real-world scenarios and essential
primitives designed to help developers build secure applications tailored to
their needs.

Currently, we have released a Web3-focused scenario, with plans to expand the
project and introduce more use cases in the future.

## Available Scenarios

- **Web3**: Available in `projects/web3/`, this scenario offers utilities for
  Web3 development, such as key custodians and decentralized identifiers (DIDs).
  It currently includes a basic Ethereum wallet that demonstrates how to
  securely create a wallet and sign transactions using wallet-derived keys
  within the TEE.

## Upcoming Scenarios

- **X509 Certificate Signing & Verification**: This scenario provides
  foundational Public Key Infrastructure (PKI) primitives for securely issuing
  self-signed certificates and verifying externally provided leaf certificates
  using a trusted certificate store. The Trusted Application (TA) inside the TEE
  handles secure key pair generation and certificate issuance, facilitating
  identity verification for secure communications. This primitive is
  particularly valuable for establishing trusted communication channels between
  nodes or devices.

- **Remote Attestation**: This foundational primitive enables remote attestation
  of a Trusted Application (TA) to ensure it is running within a Trusted
  Execution Environment (TEE). It utilizes TLS and X509 PKI to establish a
  secure communication channel.

- **Multi-Factor Authentication (MFA)**: This example demonstrates how to
  implement MFA by securely provisioning the public keys of trusted MFA devices
  (e.g., a user’s cellphone) within the Trusted Application (TA). When high-risk
  operations like key usage or transaction signing require user confirmation,
  the TA securely verifies user-provided details via the trusted MFA device,
  eliminating reliance on third-party services.
