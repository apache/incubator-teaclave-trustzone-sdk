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

# Check if IMG_DIRECTORY and IMG_NAME are provided
if [ -z "$IMG_DIRECTORY" ]; then
    echo "IMG_DIRECTORY is not set. Please set it before running this script."
    exit 1
fi
if [ -z "$IMG_DIRECTORY" ] || [ -z "$IMG_NAME" ]; then
    echo "Usage: $0 <img_directory> <img_name>"
    exit 1
fi
# Check if the image directory exists
if [ ! -d "$IMG_DIRECTORY" ]; then
    echo "Image directory does not exist: $IMG_DIRECTORY"
    echo "Creating directory: $IMG_DIRECTORY"
    mkdir -p "$IMG_DIRECTORY"
fi

${TEACLAVE_PREBUILT_DIR}/scripts/setup/download_image.sh "$IMG_DIRECTORY" "$IMG_NAME"