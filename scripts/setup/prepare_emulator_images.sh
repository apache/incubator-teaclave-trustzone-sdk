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

# Validate required environment variables
: "${IMG_DIRECTORY:?IMG_DIRECTORY must be set - directory where images will be stored}"
: "${IMG_NAME:?IMG_NAME must be set - name of the image to download}"

# Create image directory if it doesn't exist
mkdir -p "$IMG_DIRECTORY"

# Construct full image path
IMG="${IMG_DIRECTORY}/${IMG_NAME}"

# Check if the image directory exists locally
if [ ! -d "$IMG" ]; then
    echo "Image directory '$IMG' not found locally. Downloading from network."
    curl "https://nightlies.apache.org/teaclave/teaclave-trustzone-sdk/${IMG_NAME}.tar.gz" | tar zxv -C "$IMG_DIRECTORY"
else
    echo "Image directory '$IMG' found locally."
fi