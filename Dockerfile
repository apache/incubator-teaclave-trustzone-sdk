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

FROM ubuntu:16.04
MAINTAINER Teaclave Contributors <dev@teaclave.apache.org>

RUN dpkg --add-architecture i386
RUN apt-get update && \
  apt-get install -y -q android-tools-adb android-tools-fastboot autoconf \
  automake bc bison build-essential cscope curl device-tree-compiler \
  expect flex ftp-upload gdisk iasl libattr1-dev libc6:i386 libcap-dev \
  libfdt-dev libftdi-dev libglib2.0-dev libhidapi-dev libncurses5-dev \
  libpixman-1-dev libssl-dev libstdc++6:i386 libtool libz1:i386 make \
  mtools netcat python-crypto python-serial python-wand unzip uuid-dev \
  xdg-utils xterm xz-utils zlib1g-dev git wget cpio libssl-dev iasl \
  screen libbrlapi-dev libaio-dev libcurl3 libbluetooth-dev libsdl2-2.0 \
  python3 python3-pip python3-pyelftools

RUN pip3 install pycryptodome

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
  . $HOME/.cargo/env && \
  rustup default nightly-2019-07-08 && \
  rustup component add rust-src && \
  rustup target install aarch64-unknown-linux-gnu && \
  rustup default 1.44.0 && cargo +1.44.0 install xargo && \
  rustup default nightly-2019-07-08

ENV PATH="/root/.cargo/bin:$PATH"
