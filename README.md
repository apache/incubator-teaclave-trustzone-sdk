# Teaclave TrustZone SDK

[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/tag/apache/incubator-teaclave-trustzone-sdk?label=release&sort=semver)](https://github.com/apache/incubator-teaclave-trustzone-sdk/releases)
[![Homepage](https://img.shields.io/badge/site-homepage-blue)](https://teaclave.apache.org/)

Teaclave TrustZone SDK (Rust OP-TEE TrustZone SDK) provides abilities to build
safe TrustZone applications in Rust. The SDK is based on the
[OP-TEE](https://www.op-tee.org/) project which follows
[GlobalPlatform](https://globalplatform.org/) [TEE
specifications](https://globalplatform.org/specs-library/tee-internal-core-api-specification/)
and provides ergonomic APIs. In addition, it enables the capability to write
TrustZone applications with Rust's standard library (std) and many third-party
libraries (i.e., crates). Teaclave TrustZone SDK is a sub-project of [Apache
Teaclave (incubating)](https://teaclave.apache.org/).

Teaclave TrustZone SDK provides two development modes for Rust TAs: `no-std`
and `std`. 
We recommend using `no-std` by default. For a detailed comparison, please refer
to [Comparison](#comparison).

**UPDATES:** We have developed a new build environment on the `main` branch, 
which will now be the only branch for development and maintenance and includes 
breaking changes to the legacy `master` branch.
If you're using the `master` branch and wish to migrate to the new development 
branch (`main`), please refer to the 
[migration guide](docs/migrating-to-new-building-env.md).

## Table of Contents

- [TA Development Modes](#ta-development-modes)
  - [Comparison](#comparison)
  - [Supported Examples](#supported-examples)
- [Quick Start with the OP-TEE Repo for QEMUv8](#quick-start-with-the-op-tee-repo-for-qemuv8)
- [Getting Started](#getting-started)
  - [Platforms](#platforms)
    - [Develop with QEMUv8](#develop-with-qemuv8)
    - [Develop on Other Platforms](#develop-on-other-platforms)
  - [Setup Building Environment](#setup-building-environment)
  - [Build Examples](#build-examples)
  - [Run Rust Applications](#run-rust-applications)
    - [Run Rust Applications in QEMUv8](#run-rust-applications-in-qemuv8)
    - [Run Rust Applications on Other Platforms](#run-rust-applications-on-other-platforms)
  - [Test](#test)
- [Documentation](#documentation)
- [Publication](#publication)
- [Contributing](#contributing)
- [Community](#community)


## TA Development Modes

### Comparison

#### `no-std`

- **Pros**:
  - Reuses standard Rust tier-1 toolchain targets (`aarch64-unknown-linux-gnu`, 
    `arm-unknown-linux-gnueabihf`).
  - Significant performance improvements.
  - Substantial reduction in binary size.
  
- **Cons**:
  - Limited support for third-party crates. In the no-std mode, Trusted
    Applications (TAs) are unable to utilize crates dependent on the standard
    library (std).

#### `std`

- **Pros**:
  - Enables the utilization of more third-party crates, including those
    requiring `std`, such as `rustls`, which are essential for functionality.
  
- **Cons**:
  - Manual porting of `std` with infrequent updates. Currently using `std`
    version `1.80.0` and `Rust` version `nightly-2024-05-14`, which might not
    meet the MSRV requirements of some crates.

### Supported Examples

- **Common**: See
  [Overview of OP-TEE Rust Examples](https://teaclave.apache.org/trustzone-sdk-docs/overview-of-optee-rust-examples/).

- **`no-std`**: Excludes `test_serde`, `test_message_passing_interface`,
  `test_tls_client`, `test_tls_server`, `test_secure_db_abstraction`.

- **`std`**: Excludes `test_mnist_rs`, `test_build_with_optee_utee_sys`.


## Quick Start with the OP-TEE Repo for QEMUv8

Teaclave TrustZone SDK has been integrated into the OP-TEE Repo since OP-TEE
Release 3.15.0 (18/Oct/21). The aarch64 Rust examples are built and installed
into OP-TEE's default filesystem for QEMUv8. Follow [this
documentation](https://optee.readthedocs.io/en/latest/building/optee_with_rust.html)
to set up the OP-TEE repo and try the Rust examples!

UPDATES: The `no-std` TA has replaced the original `std` TAs since OP-TEE 
Release 4.1.0 (19/Jan/24).

## Getting Started

### Platforms

To get started with Teaclave TrustZone SDK, you could choose either [QEMU for
Armv8-A](#develop-with-qemuv8) (QEMUv8) or [other
platforms](#develop-on-other-platforms) ([platforms OP-TEE
supported](https://optee.readthedocs.io/en/latest/general/platforms.html)) as
your development environment.

#### Develop with QEMUv8

The OP-TEE libraries are needed when building Rust applications, so you should
finish the [Quick start with the OP-TEE Repo for
QEMUv8](#quick-start-with-the-op-tee-repo-for-qemuv8) part first. Then
initialize the building environment in Teaclave TrustZone SDK, build Rust
applications and copy them into the target's filesystem.

Teaclave TrustZone SDK is located in `[YOUR_OPTEE_DIR]/optee_rust/`. Teaclave
TrustZone SDK in OP-TEE repo is pinned to the release version. Alternatively,
you can try the develop version using `git pull`:

```sh
cd [YOUR_OPTEE_DIR]/optee_rust/
git pull github master
```

#### Develop on Other Platforms

If you are building trusted applications for other platforms ([platforms OP-TEE
supported](https://optee.readthedocs.io/en/latest/general/platforms.html)). QEMU
and the filesystem in the OP-TEE repo are not needed.  You can follow these
steps to clone the project and build applications independently from the
complete OP-TEE repo. In this case, the necessary OP-TEE libraries are
initialized in the setup process.

1. The complete list of prerequisites can be found here: [OP-TEE
Prerequisites](https://optee.readthedocs.io/en/latest/building/prerequisites.html).

   ```sh
   # install dependencies
   sudo apt-get install android-tools-adb android-tools-fastboot autoconf \
   automake bc bison build-essential ccache cscope curl device-tree-compiler \
   expect flex ftp-upload gdisk iasl libattr1-dev libc6:i386 libcap-dev \
   libfdt-dev libftdi-dev libglib2.0-dev libhidapi-dev libncurses5-dev \
   libpixman-1-dev libssl-dev libstdc++6:i386 libtool libz1:i386 make \
   mtools netcat python-crypto python3-crypto python-pyelftools \
   python3-pycryptodome python3-pyelftools python-serial python3-serial \
   rsync unzip uuid-dev xdg-utils xterm xz-utils zlib1g-dev
   ```

   Alternatively, you can use a docker container built with our
   [Dockerfile](Dockerfile).

2. After installing dependencies or building the Docker image, fetch the source
   code from the official GitHub repository:
   
   ```sh
   git clone https://github.com/apache/incubator-teaclave-trustzone-sdk.git
   cd incubator-teaclave-trustzone-sdk
   ```

### Setup Building Environment

Currently, we support building on both `aarch64` and `x86_64` host machines, and
 they share the same steps.

1. Install the Rust environment and toolchains:

   ```sh
   ./setup.sh
   ```

2. Build OP-TEE libraries

   By default, the `OPTEE_DIR` is 
   `incubator-teaclave-trustzone-sdk/optee/`. OP-TEE submodules 
   (`optee_os` and `optee_client` for QEMUv8) will be initialized 
   automatically by executing:

   ```sh
   ./build_optee_libraries.sh optee/
   ```

3. Before building applications, set up the configuration:

   a. By default, the target platform is `aarch64` for both CA and TA. If 
   you want to build for the `arm` target, you can set up `ARCH`:

   ```sh
   export ARCH_HOST=arm
   export ARCH_TA=arm
   ```

   b. By default, the build is for `no-std` TA. If you want to enable 
   `std` TA, set the `STD` variable:

   ```sh
   export STD=y
   ```

4. Run this script to set up all toolchain and library paths:

   ```sh
   source environment
   ```

### Build Examples

Run this command to build all Rust examples:

``` sh
make examples
```

Or build your own CA and TA:

```sh
make -C examples/[YOUR_APPLICATION]
```

Besides, you can collect all example CAs and TAs to
`/incubator-teaclave-trustzone-sdk/out`:

```sh
make examples-install
```

### Run Rust Applications

Considering the platform has been chosen
([QEMUv8](#run-rust-applications-in-qemuv8) or
[other](#run-rust-applications-on-other-platforms)), the ways to run the Rust
applications are different.

#### Run Rust Applications in QEMUv8

1. The shared folder is needed to share CAs and TAs with the QEMU guest system.
Recompile QEMU in OP-TEE to enable QEMU VirtFS:

```sh
(cd $OPTEE_DIR/build && make QEMU_VIRTFS_ENABLE=y qemu)
```

2. Copy all the Rust examples or your own applications to the shared folder:

```sh
mkdir shared_folder
cd [YOUR_OPTEE_DIR]/optee_rust/ && make examples-install)
cp -r [YOUR_OPTEE_DIR]/optee_rust/out/* shared_folder/
```

3. Run QEMU:

```sh
(cd $OPTEE_DIR/build && make run-only QEMU_VIRTFS_ENABLE=y
QEMU_VIRTFS_HOST_DIR=$(pwd)/shared_folder)
```

4. After the QEMU has been booted, you need to mount the shared folder in the
QEMU guest system (username: root), in order to access the compiled CA/TA from
QEMU. Run the command as follows in the QEMU guest terminal:

```sh
mkdir shared && mount -t 9p -o trans=virtio host shared
```

5. Then run CA and TA as [this
documentation](https://optee.readthedocs.io/en/latest/building/optee_with_rust.html)
describes.

#### Run Rust Applications on Other Platforms

Copy the applications to your platform and run.

### Test

In the `tests/` directory, we offer comprehensive tests for examples. The 
applications can run on a pre-built QEMU image, independently of cloning the 
OP-TEE repo. You can compose a simple test here to validate your application.

## Documentation

- [Overview of OP-TEE Rust
  Examples](https://teaclave.apache.org/trustzone-sdk-docs/overview-of-optee-rust-examples/)
- [Debugging OP-TEE
  TA](https://teaclave.apache.org/trustzone-sdk-docs/debugging-optee-ta.md/)
- [Host API
  Reference](https://teaclave.apache.org/api-docs/trustzone-sdk/optee-teec/)
- [TA API
  Reference](https://teaclave.apache.org/api-docs/trustzone-sdk/optee-utee/)

## Publication

More details about the design and implementation can be found in our paper
published in ACSAC 2020:
[RusTEE: Developing Memory-Safe ARM TrustZone
Applications](https://csis.gmu.edu/ksun/publications/ACSAC20_RusTEE_2020.pdf).
Here is the BiBTeX record for your reference.

```bibtex
@inproceedings{wan20rustee,
    author    = "Shengye Wan and Mingshen Sun and Kun Sun and Ning Zhang and Xu
He",
    title     = "{RusTEE: Developing Memory-Safe ARM TrustZone Applications}",
    booktitle = "Proceedings of the 36th Annual Computer Security Applications
Conference",
    series    = "ACSAC '20",
    year      = "2020",
    month     = "12",
}
```

## Contributing

Teaclave is open source in [The Apache
Way](https://www.apache.org/theapacheway/),
we aim to create a project that is maintained and owned by the community. All
kinds of contributions are welcome.
Thanks to our [contributors](https://teaclave.apache.org/contributors/).

## Community

- Join us on our [mailing
  list](https://lists.apache.org/list.html?dev@teaclave.apache.org).
- Follow us at [@ApacheTeaclave](https://twitter.com/ApacheTeaclave).
- See [more](https://teaclave.apache.org/community/).
