# Teaclave TrustZone SDK

Teaclave TrustZone SDK (Rust OP-TEE TrustZone SDK) provides abilities to build
safe TrustZone applications in Rust. The SDK is based on the
[OP-TEE](https://www.op-tee.org/) project which follows
[GlobalPlatform](https://globalplatform.org/) TEE specifications and provides
ergonomic APIs. In addition, it enables capability to write TrustZone
applications with Rust's standard library and many third-party libraries (i.e.,
crates). Teaclave TrustZone SDK is a sub-project of [Apache Teaclave (incubating)](https://teaclave.apache.org/).

## Getting started

To get started, you need to clone the project, initialize related submodules,
and install building dependencies (The complete list of prerequisites can be found here: [OP-TEE Prerequisites](https://optee.readthedocs.io/en/latest/building/prerequisites.html)).
Alternatively, you can use a docker container built with our [Dockerfile](Dockerfile).

``` sh
# clone the project and initialize related submodules
$ git clone git@github.com:apache/incubator-teaclave-trustzone-sdk.git
$ cd incubator-teaclave-trustzone-sdk
$ git submodule update --init
$ (cd rust/compiler-builtins && git submodule update --init libm)
$ (cd rust/rust && git submodule update --init src/stdsimd)

# install dependencies
$ sudo apt-get install android-tools-adb android-tools-fastboot autoconf \
        automake bc bison build-essential ccache cscope curl device-tree-compiler \
        expect flex ftp-upload gdisk iasl libattr1-dev libc6:i386 libcap-dev \
        libfdt-dev libftdi-dev libglib2.0-dev libhidapi-dev libncurses5-dev \
        libpixman-1-dev libssl-dev libstdc++6:i386 libtool libz1:i386 make \
        mtools netcat python-crypto python3-crypto python-pyelftools \
        python3-pycryptodome python3-pyelftools python-serial python3-serial \
        rsync unzip uuid-dev xdg-utils xterm xz-utils zlib1g-dev

# install Rust and select a proper version
$ curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2019-07-08
$ source $HOME/.cargo/env
$ rustup component add rust-src && rustup target install aarch64-unknown-linux-gnu arm-unknown-linux-gnueabihf

# install Xargo
$ rustup default 1.44.0 && cargo +1.44.0 install xargo
# switch to nightly
$ rustup default nightly-2019-07-08
```

Then, download ARM toolchains and build OP-TEE libraries. Note that the OP-TEE
target is QEMUv8, and you can modify the Makefile to other targets accordingly.

``` sh
$ make optee
```

Before building examples, the environment should be properly setup.

``` sh
$ source environment
```

By default, the target platform is `aarch64`. If you want to build for the `arm`
target, you can setup `ARCH` before source the environment like this:

```sh
$ export ARCH=arm
$ source environment
```

At last, you can get started with our examples.

``` sh
$ make examples
```

Please read detailed
[instructions](https://github.com/apache/incubator-teaclave-trustzone-sdk/wiki/Getting-started-with-OPTEE-for-QEMU-ARMv8)
to run these examples on OP-TEE for QEMU. For other supported devices, please find
more documents [here](https://optee.readthedocs.io/en/latest/general/platforms.html).

## Contributing

Teaclave TrustZone SDK is open source in [The Apache Way](https://www.apache.org/theapacheway/),
we aim to create a project that is maintained and owned by the community. All
kinds of contributions are welcome. Thanks to our [contributors](https://teaclave.apache.org/contributors/).

## Publication

More details about the design and implementation can be found in our paper
published in ACSAC 2020:
[RusTEE: Developing Memory-Safe ARM TrustZone Applications](https://csis.gmu.edu/ksun/publications/ACSAC20_RusTEE_2020.pdf).
Here is the BiBTeX record for your reference.

```
@inproceedings{wan20rustee,
    author    = "Shengye Wan and Mingshen Sun and Kun Sun and Ning Zhang and Xu He",
    title     = "{RusTEE: Developing Memory-Safe ARM TrustZone Applications}",
    booktitle = "Proceedings of the 36th Annual Computer Security Applications Conference",
    series    = "ACSAC '20",
    year      = "2020",
    month     = "12",
}
```

## License

Teaclave TrustZone SDK is distributed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
