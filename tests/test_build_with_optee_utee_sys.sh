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
cp ../examples/build_with_optee_utee_sys-rs/ta/target/$TARGET_TA/release/*.ta shared
cp ../examples/build_with_optee_utee_sys-rs/host/target/$TARGET_HOST/release/build_with_optee_utee_sys-rs shared

# Run script specific commands in QEMU
run_in_qemu "cp *.ta /lib/optee_armtz/\n"
# Run command twice, ensure the instance are keeping alive.
run_in_qemu "./build_with_optee_utee_sys-rs\n"
run_in_qemu "./build_with_optee_utee_sys-rs\n"
run_in_qemu "^C"

# Script specific checks
{
    grep -q "result is: 0" screenlog.0 &&
    grep -q "result is: 1" screenlog.0 &&
    grep -q "result is: 2" screenlog.0 &&
    grep -q "result is: 3" screenlog.0
} || {
    cat -v screenlog.0
    cat -v /tmp/serial.log
    false
}

rm screenlog.0
