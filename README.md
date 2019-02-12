# Rust OP-TEE TrustZone SDK

## Getting started

``` shell
# clone the project recursively
$ git clone --recursive git@github.com:mesalock-linux/rust-optee-trustzone-sdk.git
$ cd rust-optee-trustzone-sdk

# install dependencies
$ sudo apt-get install curl make gcc python python-crypto xz-utils

# make toolchains and OPTEE libraries
$ make

# install Rust and select a proper version
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
$ rustup default nightly-2019-02-01 && rustup component add rust-src

# install patched Xargo
$ cargo install --git https://github.com/mssun/xargo.git --branch mssun/relative-patch-path --force

# setup environment variables
$ source environment
```
