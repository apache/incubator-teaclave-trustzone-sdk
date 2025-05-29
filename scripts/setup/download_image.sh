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

# This script downloads an image file to a specified directory if it does not already exist.

set -xe

IMG_DIRECTORY=$1
IMG_NAME=$2
IMG="${IMG_DIRECTORY}/${IMG_NAME}"

# Check if the image file exists locally
if [ ! -d "${IMG}" ]; then
    echo "Image file '${IMG}' not found locally. Downloading from network."
    curl "https://nightlies.apache.org/teaclave/teaclave-trustzone-sdk/${IMG_NAME}.tar.gz" | tar zxv -C "$IMG_DIRECTORY"

else
    echo "Image file '${IMG}' found locally."
fi
