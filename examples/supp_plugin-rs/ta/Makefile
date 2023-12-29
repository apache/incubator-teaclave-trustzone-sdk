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

UUID ?= $(shell cat "../ta_uuid.txt")

ARCH ?= aarch64

OBJCOPY := $(CROSS_COMPILE)objcopy
TA_SIGN_KEY ?= $(TA_DEV_KIT_DIR)/keys/default_ta.pem
SIGN := $(TA_DEV_KIT_DIR)/scripts/sign_encrypt.py

ifeq ($(ARCH), arm)
	TARGET := arm-unknown-linux-gnueabihf
	LINKER_CFG := target.arm-unknown-linux-gnueabihf.linker=\"$(CROSS_COMPILE)ld.bfd\"
else
	TARGET := aarch64-unknown-linux-gnu
	LINKER_CFG := target.aarch64-unknown-linux-gnu.linker=\"$(CROSS_COMPILE)ld.bfd\"
endif

OUT_DIR := $(CURDIR)/target/$(TARGET)/release

all: ta strip sign

ta:
	@cargo build --target $(TARGET) --release --config $(LINKER_CFG)

strip:
	@$(OBJCOPY) --strip-unneeded $(OUT_DIR)/ta $(OUT_DIR)/stripped_ta

sign:
	@$(SIGN) --uuid $(UUID) --key $(TA_SIGN_KEY) --in $(OUT_DIR)/stripped_ta --out $(OUT_DIR)/$(UUID).ta
	@echo "SIGN =>  ${UUID}"

clean:
	@cargo clean
