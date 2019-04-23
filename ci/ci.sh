#!/bin/bash

set -xe

pushd ../tests

./test_hello_world.sh
./test_random.sh
./test_secure_storage.sh
./test_aes.sh
./test_serde.sh
./test_hotp.sh
./test_acipher.sh

popd
