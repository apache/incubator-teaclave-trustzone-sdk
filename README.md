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
# install dependencies
$ sudo apt-get install android-tools-adb android-tools-fastboot autoconf \
        automake bc bison build-essential ccache cscope curl device-tree-compiler \
        expect flex ftp-upload gdisk iasl libattr1-dev libc6:i386 libcap-dev \
        libfdt-dev libftdi-dev libglib2.0-dev libhidapi-dev libncurses5-dev \
        libpixman-1-dev libssl-dev libstdc++6:i386 libtool libz1:i386 make \
        mtools netcat python-crypto python3-crypto python-pyelftools \
        python3-pycryptodome python3-pyelftools python-serial python3-serial \
        rsync unzip uuid-dev xdg-utils xterm xz-utils zlib1g-dev

# clone the project
$ git clone git@github.com:apache/incubator-teaclave-trustzone-sdk.git
$ cd incubator-teaclave-trustzone-sdk
# initialize related submodules and install Rust environment
$ ./setup.sh
```

By default, the `OPTEE_DIR` is `incubator-teaclave-trustzone-sdk/optee/`.
``` sh
# initialize OP-TEE submodule
$ git submodule update --init -- optee
```
If you already have [OP-TEE repository](https://github.com/OP-TEE) 
cloned, you can set OP-TEE root directory:

``` sh
$ export OPTEE_DIR=path/to/your/optee/root/directory
```

Note that your OPTEE root directory should have `build/`, `optee_os/` and 
`optee_client/` as sub-directory.

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

Then, download ARM toolchains and build OP-TEE libraries. Note that the OP-TEE
target is QEMUv8, and you can modify the Makefile to other targets accordingly.

``` sh
$ make optee
```

At last, you can get started with our examples.

``` sh
$ make examples
```

Please read detailed
[instructions](https://teaclave.apache.org/trustzone-sdk-docs/getting-started-with-optee-for-qemu-armv8/)
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
