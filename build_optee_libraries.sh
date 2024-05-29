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

set -e

OPTEE_VERSION=4.2.0

TARGET_PATH=$1

# check arguments
if [ -z "$TARGET_PATH" ]; then
    echo "Usage: $0 <path_for_storage>"
    exit 1
fi

# set toolchain

HOST_ARCH=$(uname -m)

if [ "$HOST_ARCH" == "aarch64" ]; then
    export CROSS_COMPILE=""
    export CROSS_COMPILE64=""
else
    export CROSS_COMPILE="aarch64-linux-gnu-"
    export CROSS_COMPILE64="aarch64-linux-gnu-"
fi
export CROSS_COMPILE32="arm-linux-gnueabihf-"

# build optee_os and optee_client for qemu_v8
git clone https://github.com/OP-TEE/optee_os.git -b $OPTEE_VERSION $TARGET_PATH/optee_os
(cd $TARGET_PATH/optee_os && make PLATFORM=vexpress-qemu_armv8a)

git clone https://github.com/OP-TEE/optee_client.git -b $OPTEE_VERSION $TARGET_PATH/optee_client
(cd $TARGET_PATH/optee_client && make WITH_TEEACL=0)
