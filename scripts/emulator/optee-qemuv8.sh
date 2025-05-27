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

# Define IMG_VERSION
IMG_VERSION="$(uname -m)-optee-qemuv8-ubuntu-24.04"
IMG_DIRECTORY=${TEACLAVE_PREBUILT_DIR}/images
# Default value for NEED_EXPANDED_MEM
: ${NEED_EXPANDED_MEM:=false}

# Set IMG based on NEED_EXPANDED_MEM
if [ "$NEED_EXPANDED_MEM" = true ]; then
    IMG_NAME="${IMG_VERSION}-expand-ta-memory"
else
    IMG_NAME="$IMG_VERSION"
fi

mkdir -p "${IMG_DIRECTORY}"
${TEACLAVE_SRC_DIR}/scripts/emulator/download_image.sh "$IMG_DIRECTORY" "$IMG_NAME"

# if DEBUG is set, use this serial commands: -serial tcp:localhost:54320 -serial tcp:localhost:54321
# e.g. DEBUG=1 ./optee-qemuv8.sh
SERIAL_CMDS=""
if [ -n "$DEBUG" ]; then
    # before running this script, run the following commands in two separate terminals for listening to the serial output:
    # docker exec -it elegant_hertz /teaclave/scripts/emulator/boot_guest_rootfs.exp
    # docker exec -it elegant_hertz /teaclave/scripts/emulator/listen_on_ta_output.sh
    SERIAL_CMDS="-serial tcp:localhost:54320 -serial tcp:localhost:54321"
else
    # Default serial commands for non-debug mode
    # Guest vm output is in standard output, and TA serial log is saved to /tmp/serial.log
    SERIAL_CMDS="-serial stdio -serial file:/tmp/serial.log"
fi

IMG="${IMG_DIRECTORY}/${IMG_NAME}"

cd ${IMG} && ./qemu-system-aarch64 \
    -nodefaults \
    -nographic \
    $SERIAL_CMDS \
    -smp 2 \
    -s -machine virt,secure=on,acpi=off,gic-version=3 \
    -cpu cortex-a57 \
    -d unimp -semihosting-config enable=on,target=native \
    -m 1057 \
    -bios bl1.bin \
    -initrd rootfs.cpio.gz \
    -append 'console=ttyAMA0,115200 keep_bootcon root=/dev/vda2' \
    -kernel Image \
    -fsdev local,id=fsdev0,path=${QEMU_HOST_SHARE_DIR},security_model=none \
    -device virtio-9p-device,fsdev=fsdev0,mount_tag=host \
    -netdev user,id=vmnic,hostfwd=:127.0.0.1:54433-:4433 \
    -device virtio-net-device,netdev=vmnic
