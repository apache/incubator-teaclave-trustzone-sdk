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

NEED_EXPANDED_MEM=true
# Include base script
source setup.sh

rm -f openssl.log

# Copy TA and host binary
cp ../examples/tls_server-rs/ta/target/$TARGET_TA/release/*.ta shared
cp ../examples/tls_server-rs/host/target/$TARGET_HOST/release/tls_server-rs shared
cp ../examples/tls_server-rs/ta/test-ca/ecdsa/ca.cert shared

# Run script specific commands in QEMU
run_in_qemu "cp *.ta /lib/optee_armtz/\n"
run_in_qemu "./tls_server-rs\n"
# Outside the QEMU: connect the server using openssl, accept the self-signed CA cert
# || true because we want to continue testing even if the connection fails, and check the log later
echo "Q" | openssl s_client -connect 127.0.0.1:54433 -CAfile shared/ca.cert -debug > openssl.log 2>&1 || true
run_in_qemu "^C"

# Script specific checks
{
	grep -q "Verification: OK" openssl.log &&
	grep -q "close session" screenlog.0
} || {
	cat -v screenlog.0
	cat -v /tmp/serial.log
	cat -v openssl.log
	false
}

rm screenlog.0

rm -rf openssl.log