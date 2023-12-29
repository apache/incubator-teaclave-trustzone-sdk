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
# install rustup and stable Rust if needed
if command -v rustup &>/dev/null ; then
	rustup install stable
else
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	source "$HOME/.cargo/env"
fi

# Ensure the toolchain, components, and targets we've specified in
# rust-toolchain.toml are ready to go. Since that file sets rustup's default
# toolchain for the entire directory, all we need to do is run any
# rustup-wrapped command to trigger installation. We've arbitrarily chosen
# "cargo --version" since it has no other effect.
cargo --version >/dev/null
