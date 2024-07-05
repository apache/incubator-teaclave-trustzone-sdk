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
cp ../examples/acipher-rs/ta/target/$TARGET_TA/release/*.ta shared
cp ../examples/acipher-rs/host/target/$TARGET_HOST/release/acipher-rs shared

# Run script specific commands in QEMU
run_in_qemu "cp *.ta /lib/optee_armtz/\n"
run_in_qemu "./acipher-rs 256 teststring\n"
run_in_qemu "^C"

# Script specific checks
{
    grep -q "Success encrypt input text \".*\" as [0-9]* bytes cipher text:" screenlog.0 &&
    grep -q "Success decrypt the above ciphertext as [0-9]* bytes plain text:" screenlog.0
} || {
    cat -v screenlog.0
    cat -v /tmp/serial.log
    false
}

rm screenlog.0
