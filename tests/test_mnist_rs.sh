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

# Include base script
source setup.sh

# Copy TA and host binary
cp ../examples/mnist-rs/ta/target/$TARGET_TA/release/*.ta shared
cp ../examples/mnist-rs/host/target/$TARGET_HOST/release/mnist-rs shared
# Copy samples files
cp -r ../examples/mnist-rs/host/samples shared

# Run script specific commands in QEMU
run_in_qemu "cp *.ta /lib/optee_armtz/\n"
# Do not export the model due to QEMU's memory limitations.
run_in_qemu_with_timeout_secs "./mnist-rs train -n 1\n" 300
run_in_qemu "./mnist-rs infer -m samples/model.bin -b samples/7.bin -i samples/7.png\n"
run_in_qemu "^C"

# Script specific checks
{
    grep -q "Train Success" screenlog.0 &&
    grep -q "Infer Success" screenlog.0
} || {
    cat -v screenlog.0
    cat -v /tmp/serial.log
    false
}

rm screenlog.0
