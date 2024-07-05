#!/bin/bash

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

set -xe

##########################################
# move to project root
cd "$(dirname "$0")"

##########################################
# install Xargo
cargo +stable install xargo

##########################################
# initialize submodules: rust / libc
RUST_COMMIT_ID=7ee181c5199b0769414f0d0fd13f5e959ef84c27
LIBC_COMMIT_ID=4fa30318ed3175f6ebe22da8f167f9f9b34567c3

if [ -d rust/ ]
then
	rm -r rust/
fi

mkdir rust && cd rust

git clone https://github.com/DemesneGH/rust.git && \
	(cd rust && \
	git checkout "$RUST_COMMIT_ID" && \
	git submodule update --init library/stdarch && \
	git submodule update --init library/backtrace)

git clone https://github.com/DemesneGH/libc.git && \
	(cd libc && \
	git checkout "$LIBC_COMMIT_ID")

echo "Rust submodules initialized"
