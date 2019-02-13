# Rust OP-TEE TrustZone SDK

## Getting started

``` shell
# clone the project and initialize related submodules
$ git clone git@github.com:mesalock-linux/rust-optee-trustzone-sdk.git
$ cd rust-optee-trustzone-sdk
$ git submodule update --init
$ (cd rust/compiler-builtins && git submodule update --init libm compiler-rt)
$ (cd rust/rust && git submodule update --init src/stdsimd)

# install dependencies
$ sudo apt-get install curl make gcc python python-crypto xz-utils

# make toolchains and OPTEE libraries
$ make optee

# install Rust and select a proper version
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
$ rustup default nightly-2019-02-01 && rustup component add rust-src
$ rustup target install aarch64-unknown-linux-gnu

# install patched Xargo
$ cargo install --git https://github.com/mssun/xargo.git --branch mssun/relative-patch-path --force

# setup environment variables
$ source environment

# make all examples
$ make examples
```
