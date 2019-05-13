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
./test_big_int.sh
./test_diffie_hellman.sh

popd
