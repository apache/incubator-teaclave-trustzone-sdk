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

rm -rf ecdsa/
mkdir -p ecdsa/

# Create temporary OpenSSL configuration
cat > ecdsa/temp_openssl.cnf << 'EOF'
[ v3_end ]
basicConstraints = critical,CA:false
keyUsage = nonRepudiation, digitalSignature
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always,issuer:always
subjectAltName = @alt_names

[ v3_inter ]
subjectKeyIdentifier = hash
extendedKeyUsage = critical, serverAuth, clientAuth
basicConstraints = CA:true
keyUsage = cRLSign, keyCertSign, digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment, keyAgreement, keyCertSign, cRLSign

[ alt_names ]
DNS.1 = testserver.com
DNS.2 = second.testserver.com
DNS.3 = localhost
EOF

# Generate ECDSA parameter files
openssl ecparam -name prime256v1 -out ecdsa/nistp256.pem
openssl ecparam -name secp384r1 -out ecdsa/nistp384.pem

# Generate CA certificate
openssl req -nodes \
          -x509 \
          -newkey ec:ecdsa/nistp384.pem \
          -keyout ecdsa/ca.key \
          -out ecdsa/ca.cert \
          -sha256 \
          -batch \
          -days 3650 \
          -subj "/CN=testserver"

# Generate intermediate certificate request
openssl req -nodes \
          -newkey ec:ecdsa/nistp256.pem \
          -keyout ecdsa/inter.key \
          -out ecdsa/inter.req \
          -sha256 \
          -batch \
          -subj "/CN=testserver"

# Generate end certificate request
openssl req -nodes \
          -newkey ec:ecdsa/nistp256.pem \
          -keyout ecdsa/end.key \
          -out ecdsa/end.req \
          -sha256 \
          -batch \
          -subj "/CN=testserver"

# Sign intermediate certificate with CA
openssl x509 -req \
          -in ecdsa/inter.req \
          -out ecdsa/inter.cert \
          -CA ecdsa/ca.cert \
          -CAkey ecdsa/ca.key \
          -sha256 \
          -days 3650 \
          -set_serial 123 \
          -extensions v3_inter -extfile ecdsa/temp_openssl.cnf

# Sign end certificate with intermediate
openssl x509 -req \
          -in ecdsa/end.req \
          -out ecdsa/end.cert \
          -CA ecdsa/inter.cert \
          -CAkey ecdsa/inter.key \
          -sha256 \
          -days 2000 \
          -set_serial 456 \
          -extensions v3_end -extfile ecdsa/temp_openssl.cnf

# Create certificate chains
cat ecdsa/inter.cert ecdsa/ca.cert > ecdsa/end.chain
cat ecdsa/end.cert ecdsa/inter.cert ecdsa/ca.cert > ecdsa/end.fullchain

# Create DER format CA certificate
openssl asn1parse -in ecdsa/ca.cert -out ecdsa/ca.der > /dev/null

# Clean up temporary configuration file
rm -f ecdsa/temp_openssl.cnf

echo "ECDSA test certificates generated successfully."