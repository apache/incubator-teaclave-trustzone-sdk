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

# sync.sh <project_path>: copy the built project files to the QEMU share folder for futher emulating

set -xe

# Check if the project path is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <project_path>"
    exit 1
fi
PROJECT_PATH="$1"

# Check if the $QEMU_HOST_SHARE_DIR is existed
if [ -z "$QEMU_HOST_SHARE_DIR" ]; then
    echo "QEMU_HOST_SHARE_DIR is not set. Please set it before running this script."
    exit 1
fi
if [ ! -d "$QEMU_HOST_SHARE_DIR" ]; then
    echo "QEMU_HOST_SHARE_DIR does not exist: $QEMU_HOST_SHARE_DIR"
    exit 1
fi

make -C "$PROJECT_PATH" install DESTDIR="$QEMU_HOST_SHARE_DIR"
if [ $? -ne 0 ]; then
    echo "Make install failed in $PROJECT_PATH"
    exit 1
fi
echo "Successfully synced project to $QEMU_HOST_SHARE_DIR"