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

# Functions for running commands in QEMU screen
run_in_qemu() {
    (screen -S qemu_screen -p 0 -X stuff "$1\n") || (echo "run_in_qemu '$1' failed" && cat /tmp/serial.log)
    sleep 5
}

run_in_qemu_with_timeout_secs() {
    (screen -S qemu_screen -p 0 -X stuff "$1\n") || (echo "run_in_qemu '$1' failed" && cat /tmp/serial.log)
    sleep $2
}

# Start QEMU screen
screen -L -d -m -S qemu_screen $TEACLAVE_SRC_DIR/scripts/emulator/optee-qemuv8.sh
sleep 30
run_in_qemu "root"
run_in_qemu "mkdir -p shared && mount -t 9p -o trans=virtio host shared && cd shared"
# libteec.so.2 since OP-TEE 4.2.0, for legacy versions:
run_in_qemu "[ ! -e /usr/lib/libteec.so.1 ] && ln -s /usr/lib/libteec.so /usr/lib/libteec.so.1"
