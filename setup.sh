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

# install Xargo
cargo +stable install xargo

########################################################
# initialize submodules: optee_os / optee_client / build
OPTEE_RELEASE_VERSION=4.2.0

if [[ -z "$OPTEE_DIR" ]] || [[ "$OPTEE_DIR" == "$(pwd)/optee" ]]
then
	OPTEE_DIR="$(pwd)/optee"
	echo "optee dir: $OPTEE_DIR"
	OPTEE_SUBMODULES=(optee_os optee_client build)

	if [ ! -d "$OPTEE_DIR" ]
	then
		mkdir "$OPTEE_DIR"
	else
		rm -r "$OPTEE_DIR"/*
	fi

	# download optee release
	echo "Downloading optee release..."
	for submodule in ${OPTEE_SUBMODULES[*]}
	do
		echo "Downloading $submodule..."
		curl --retry 5 -s -S \
			-L "https://github.com/OP-TEE/$submodule/archive/refs/tags/$OPTEE_RELEASE_VERSION.tar.gz" \
			-o "$OPTEE_DIR/$OPTEE_RELEASE_VERSION.tar.gz"
		if [ ! $? -eq 0 ]
		then
			rm "$OPTEE_DIR/$OPTEE_RELEASE_VERSION.tar.gz" && \
				echo "Download failed" && \
				exit 1
		fi
		echo "Uncompressing $submodule..."
		mkdir -p "$OPTEE_DIR/$submodule" && \
			tar zxf "$OPTEE_DIR/$OPTEE_RELEASE_VERSION.tar.gz" \
			-C "$OPTEE_DIR/$submodule" --strip-components 1
		if [ ! $? -eq 0 ]
		then
			rm "$OPTEE_DIR/$OPTEE_RELEASE_VERSION.tar.gz" && \
				rm -r "$OPTEE_DIR/$submodule" && \
				echo "Downloaded file is damaged" && \
				exit 1
		fi
		rm "$OPTEE_DIR/$OPTEE_RELEASE_VERSION.tar.gz"
	done
	echo "Download finished"
else
	echo "OPTEE_DIR has been set, omit to download optee submodules"
fi


########################################################
# initialize submodules: rust / compiler-builtins / libc
RUST_COMMIT_ID=cb8a61693c80ebc615c2b66f40f0789cd16e699a
COMPILER_BUILTINS_COMMIT_ID=45a2e4996fe732172004b292b07397f9a02265ab
LIBC_COMMIT_ID=1ddfbbbc190bec0f5ec32b08e97585b34d0c6b09

if [ -d rust/ ]
then
	rm -r rust/
fi

mkdir rust && cd rust

git clone https://github.com/mesalock-linux/rust.git && \
	(cd rust && \
	git checkout "$RUST_COMMIT_ID" && \
	git submodule update --init library/stdarch && \
	git submodule update --init library/backtrace)

git clone https://github.com/mesalock-linux/compiler-builtins.git && \
	(cd compiler-builtins && \
	git checkout "$COMPILER_BUILTINS_COMMIT_ID" && \
	git submodule update --init libm)

git clone https://github.com/mesalock-linux/libc.git && \
	(cd libc && \
	git checkout "$LIBC_COMMIT_ID")

echo "Rust submodules initialized"
