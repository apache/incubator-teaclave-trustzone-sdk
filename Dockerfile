# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

FROM ubuntu:20.04
MAINTAINER Teaclave Contributors <dev@teaclave.apache.org>
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies for building OP-TEE
RUN apt-get update && \
    apt-get install -y \
    git \
    android-tools-adb \
    android-tools-fastboot \
    autoconf \
    automake \
    bc \
    bison \
    build-essential \
    ccache \
    cscope \
    curl \
    device-tree-compiler \
    expect \
    flex \
    ftp-upload \
    gdisk \
    iasl \
    libattr1-dev \
    libcap-dev \
    libfdt-dev \
    libftdi-dev \
    libglib2.0-dev \
    libgmp-dev \
    libhidapi-dev \
    libmpc-dev \
    libncurses5-dev \
    libpixman-1-dev \
    libssl-dev \
    libtool \
    make \
    mtools \
    netcat \
    ninja-build \
    python \
    python-crypto \
    python3-crypto \
    python-pyelftools \
    python3-pycryptodome \
    python3-pyelftools \
    python3-serial \
    rsync \
    unzip \
    uuid-dev \
    xdg-utils \
    xterm \
    xz-utils \
    zlib1g-dev \
    wget \
    cpio \
    libcap-ng-dev \
    screen \
    libvdeplug-dev \
    libsdl2-dev \
    pip \
    ca-certificates

RUN pip install cryptography

RUN apt-get install -y software-properties-common && \
    add-apt-repository ppa:linuxuprising/libpng12 && \
    apt-get update && \
    apt-get install libpng12-0

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
  . $HOME/.cargo/env && \
  rustup default nightly-2021-09-20 && \
  rustup component add rust-src && \
  rustup target install aarch64-unknown-linux-gnu && \
  rustup default 1.44.0 && cargo +1.44.0 install xargo && \
  rustup default nightly-2021-09-20

ENV PATH="/root/.cargo/bin:$PATH"
