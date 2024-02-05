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

rm -rf screenlog.0
rm -rf optee-qemuv8-3.20.0-ubuntu-20.04
rm -rf shared

curl https://nightlies.apache.org/teaclave/teaclave-trustzone-sdk/optee-qemuv8-3.20.0-ubuntu-20.04.tar.gz | tar zxv
mkdir shared
cp ../examples/random-rs/ta/target/aarch64-unknown-linux-gnu/release/*.ta shared
cp ../examples/random-rs/host/target/aarch64-unknown-linux-gnu/release/random-rs shared

screen -L -d -m -S qemu_screen ./optee-qemuv8.sh
sleep 30
screen -S qemu_screen -p 0 -X stuff "root\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "mkdir shared && mount -t 9p -o trans=virtio host shared && cd shared\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "cp *.ta /lib/optee_armtz/\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "./random-rs\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "^C"
sleep 5

{
	grep -q "Invoking TA to generate random UUID" screenlog.0 &&
	grep -q "Generate random UUID: [a-z0-9]*-[a-z0-9]*-[a-z0-9]*-[a-z0-9]*" screenlog.0 &&
	grep -q "Success" screenlog.0
} || {
        cat -v screenlog.0
        cat -v /tmp/serial.log
        false
}

rm -rf screenlog.0
rm -rf optee-qemuv8-3.20.0-ubuntu-20.04
rm -rf shared
