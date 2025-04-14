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

# Default value for NEED_EXPANDED_MEM
: ${NEED_EXPANDED_MEM:=false}

# Define IMG_VERSION
IMG_VERSION="$(uname -m)-optee-qemuv8-ubuntu-24.04"

# Set IMG based on NEED_EXPANDED_MEM
if [ "$NEED_EXPANDED_MEM" = true ]; then
    IMG="${IMG_VERSION}-expand-ta-memory"
else
    IMG="$IMG_VERSION"
fi

# Function to download image
download_image() {
    curl "https://nightlies.apache.org/teaclave/teaclave-trustzone-sdk/${IMG}.tar.gz" | tar zxv
}

# Functions for running commands in QEMU screen
run_in_qemu() {
    (screen -S qemu_screen -p 0 -X stuff "$1\n") || (echo "run_in_qemu '$1' failed" && cat /tmp/serial.log)
    sleep 5
}

run_in_qemu_with_timeout_secs() {
    (screen -S qemu_screen -p 0 -X stuff "$1\n") || (echo "run_in_qemu '$1' failed" && cat /tmp/serial.log)
    sleep $2
}

# Check if the image file exists locally
if [ ! -d "${IMG}" ]; then
    echo "Image file '${IMG}' not found locally. Downloading from network."
    download_image
else
    echo "Image file '${IMG}' found locally."
fi

mkdir -p shared

# Start QEMU screen
screen -L -d -m -S qemu_screen ./optee-qemuv8.sh $IMG
sleep 30
run_in_qemu "root"
run_in_qemu "mkdir -p shared && mount -t 9p -o trans=virtio host shared && cd shared"
# libteec.so.2 since OP-TEE 4.2.0, for legacy versions:
run_in_qemu "[ ! -e /usr/lib/libteec.so.1 ] && ln -s /usr/lib/libteec.so /usr/lib/libteec.so.1"
