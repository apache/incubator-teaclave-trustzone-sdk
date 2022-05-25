# Teaclave TrustZone SDK

[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/tag/apache/incubator-teaclave-trustzone-sdk?label=release&sort=semver)](https://github.com/apache/incubator-teaclave-trustzone-sdk/releases)
[![Homepage](https://img.shields.io/badge/site-homepage-blue)](https://teaclave.apache.org/)

Teaclave TrustZone SDK (Rust OP-TEE TrustZone SDK) provides abilities to build
safe TrustZone applications in Rust. The SDK is based on the
[OP-TEE](https://www.op-tee.org/) project which follows
[GlobalPlatform](https://globalplatform.org/) TEE specifications and provides
ergonomic APIs. In addition, it enables capability to write TrustZone
applications with Rust's standard library and many third-party libraries (i.e.,
crates). Teaclave TrustZone SDK is a sub-project of [Apache Teaclave
(incubating)](https://teaclave.apache.org/).

## Getting started

### Quick start with the OP-TEE Repo for QEMUv8

Teaclave TrustZone SDK has been integrated into the OP-TEE Repo since OP-TEE
Release 3.15.0 (18/Oct/21). The aarch64 Rust examples are built and installed
into OP-TEE's default filesystem for QEMUv8. Follow [this
documentation](https://optee.readthedocs.io/en/latest/building/optee_with_rust.html)
to set up the OP-TEE repo and try the Rust examples!

### Develop your trusted applications in Rust

The OP-TEE libraries are needed when building Rust applications, so you should
finish the `Quick start with the OP-TEE Repo for QEMUv8` part first. Then
initialize the building environment in Teaclave TrustZone SDK, build Rust
applications and copy them into the target's filesystem.

#### 1. Update the project 

Teaclave TrustZone SDK is located in `YOUR_OPTEE_DIR/optee_rust/`.Teaclave
TrustZone SDK in OP-TEE repo is pinned to the release version. Alternatively,
you can try the develop version using `git pull`:

```
$ cd [YOUR_OPTEE_DIR]/optee_rust/
$ git pull github master
```

#### 2. Install Rust environment and initialize related submodules

* Set the OP-TEE root directory:

``` sh
$ export OPTEE_DIR=[YOUR_OPTEE_DIR]
```

* Run the script as follows to install Rust environment and initialize
  submodules:

```
$ ./setup.sh
```

#### 3. Set environment variables

Before building examples, the environment should be properly setup.

``` sh
$ source environment
```

By default, the target platform is `aarch64`. If you want to build for the `arm`
target, you can setup `ARCH` before `source environment`:

```sh
$ export ARCH=arm
$ source environment
```

#### 4. Build Rust applications

Before building built-in rust examples, you need to build OP-TEE libraries using:

``` sh
$ make optee
```

Run this command to build all Rust examples:

``` sh
$ make examples
```

Or build your own CA and TA:

```
$ make -C examples/[YOUR_APPLICATION]
```

#### 5. Run Rust applications

The shared folder is needed to share CAs and TAs with the QEMU guest system.
Recompile QEMU in OP-TEE to enable QEMU virtfs:

```
$ (cd $OPTEE_DIR/build && make QEMU_VIRTFS_ENABLE=y qemu)
```

Note: the path `/project/root/dir/` should be replaced as the root directory of
your local project "Teaclave TrustZone SDK". Copy all the Rust examples or your
own applications to the shared folder:

```sh
$ mkdir shared_folder
$ (cd /project/root/dir/ && make examples-install)
$ cp -r /project/root/dir/out/* shared_folder/
```

Run QEMU.

```sh
$ (cd $OPTEE_DIR/build && make run-only QEMU_VIRTFS_ENABLE=y
QEMU_VIRTFS_HOST_DIR=$(pwd)/shared_folder)
```

After the QEMU has been booted, you need to mount the shared folder in QEMU
guest system (username: root), in order to access the compiled CA/TA from QEMU.
Run the command as follows in the QEMU guest terminal:

```sh
$ mkdir shared && mount -t 9p -o trans=virtio host shared
```

Then run CA and TA as 
[this documentation](https://optee.readthedocs.io/en/latest/building/optee_with_rust.html)
 describes.

## Use OP-TEE libraries as submodules

If you are building trusted applications for other platforms ([platforms OP-TEE
supported](https://optee.readthedocs.io/en/latest/general/platforms.html)). QEMU
and the filesystem in OP-TEE repo are not needed.  You can follow these steps to
clone the project and build applications independently from the complete OP-TEE
repo. In this case, the necessary OP-TEE libraries are initialized in the setup
process.

#### 1. Clone the project and install building dependencies

The complete list of prerequisites can be found here: [OP-TEE
Prerequisites](https://optee.readthedocs.io/en/latest/building/prerequisites.html).

Alternatively, you can use a docker container built with our
[Dockerfile](Dockerfile).

``` sh
# install dependencies
$ sudo apt-get install android-tools-adb android-tools-fastboot autoconf \
	automake bc bison build-essential ccache cscope curl device-tree-compiler \
	expect flex ftp-upload gdisk iasl libattr1-dev libc6:i386 libcap-dev \
	libfdt-dev libftdi-dev libglib2.0-dev libhidapi-dev libncurses5-dev \
	libpixman-1-dev libssl-dev libstdc++6:i386 libtool libz1:i386 make \
	mtools netcat python-crypto python3-crypto python-pyelftools \
	python3-pycryptodome python3-pyelftools python-serial python3-serial \
	rsync unzip uuid-dev xdg-utils xterm xz-utils zlib1g-dev
```

After installing dependencies or building the Docker container, fetch the source code from the official GitHub repository:

``` sh
# clone the project
$ git clone https://github.com/apache/incubator-teaclave-trustzone-sdk.git
$ cd incubator-teaclave-trustzone-sdk
```

#### 2. Set your OP-TEE directory

* By default, the `OPTEE_DIR` is
  `incubator-teaclave-trustzone-sdk/optee/`.OP-TEE submodules (`optee_os`,
`optee_client` and` build`) will be initialized automatically in `setup.sh`. If
you already have [OP-TEE repository](https://github.com/OP-TEE)  cloned
somewhere, you can set OP-TEE root directory:

``` sh
$ export OPTEE_DIR=[YOUR_OPTEE_DIR]
```

Note that your OPTEE root directory should have `build/`, `optee_os/` and 
`optee_client/` as sub-directory.

* Run the script as follows to install Rust environment and set up submodules.

```
$ ./setup.sh
```

#### 3. Set environment variables

Before building examples, the environment should be properly setup.

``` sh
$ source environment
```

By default, the target platform is `aarch64`. If you want to build for the `arm`
target, you can setup `ARCH` before `source environment` like this:

```sh
$ export ARCH=arm
$ source environment
```

#### 4. Build OP-TEE libraries

Then, download ARM toolchains and build OP-TEE libraries. Note that the OP-TEE
target is QEMUv8, and you can modify the Makefile to other targets accordingly.

``` sh
$ make optee
```

#### 5. Build Rust examples

Run this command to build all Rust examples:

``` sh
$ make examples
```

Collect all example CAs and TAs to `/incubator-teaclave-trustzone-sdk/out`:

``` sh
$ make examples-install
```

#### 6. Run Rust examples

Copy the applications to your platform and run.


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

```
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
