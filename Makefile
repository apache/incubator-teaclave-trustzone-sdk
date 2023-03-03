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

OPTEE_PATH        ?= $(OPTEE_DIR)
OPTEE_BUILD_PATH  ?= $(OPTEE_PATH)/build
OPTEE_OS_PATH     ?= $(OPTEE_PATH)/optee_os
OPTEE_CLIENT_PATH ?= $(OPTEE_PATH)/optee_client

CCACHE ?= $(shell which ccache)

EXAMPLES = $(wildcard examples/*)
EXAMPLES_INSTALL = $(EXAMPLES:%=%-install)
EXAMPLES_CLEAN  = $(EXAMPLES:%=%-clean)

ifneq ($(ARCH), arm)
	VENDOR := qemu_v8.mk
	AARCH_CROSS_COMPILE := $(OPTEE_PATH)/toolchains/aarch64/bin/aarch64-linux-gnu-
	HOST_TARGET := aarch64-unknown-linux-gnu
	TA_TARGET := aarch64-unknown-optee-trustzone
else
	VENDOR := qemu.mk
	ARCH_CROSS_COMPILE := $(OPTEE_PATH)/toolchains/aarch32/bin/arm-linux-gnueabihf-
	HOST_TARGET := arm-unknown-linux-gnueabihf
	TA_TARGET := arm-unknown-optee-trustzone
endif

all: toolchains optee-os optee-client examples
optee: toolchains optee-os optee-client

toolchains:
	make -C $(OPTEE_BUILD_PATH) -f $(VENDOR) toolchains

optee-os:
	make -C $(OPTEE_BUILD_PATH) -f $(VENDOR) optee-os

OPTEE_CLIENT_FLAGS ?= CROSS_COMPILE="$(CCACHE) $(AARCH_CROSS_COMPILE)" \
	CFG_TEE_BENCHMARK=n \
	CFG_TA_TEST_PATH=y \
	WITH_TEEACL=0

optee-client:
	make -C $(OPTEE_CLIENT_PATH) $(OPTEE_CLIENT_FLAGS)

examples: $(EXAMPLES) toolchains optee-os optee-client
$(EXAMPLES):
	make -C $@

examples-install: $(EXAMPLES_INSTALL)
$(EXAMPLES_INSTALL):
	install -D $(@:%-install=%)/host/target/$(HOST_TARGET)/release/$(@:examples/%-install=%) -t out/host/
	install -D $(@:%-install=%)/ta/target/$(TA_TARGET)/release/*.ta -t out/ta/
	if [ -d "$(@:%-install=%)/plugin/target/" ]; then \
		install -D $(@:%-install=%)/plugin/target/$(HOST_TARGET)/release/*.plugin.so -t out/plugin/; \
	fi

optee-os-clean:
	make -C $(OPTEE_OS_PATH) O=out/arm clean

optee-client-clean:
	make -C $(OPTEE_CLIENT_PATH) $(OPTEE_CLIENT_CLEAN_FLAGS) clean

examples-clean: $(EXAMPLES_CLEAN) out-clean
$(EXAMPLES_CLEAN):
	make -C $(@:-clean=) clean

out-clean:
	rm -rf out

.PHONY: clean optee-os-clean optee-client-clean $(EXAMPLES) $(EXAMPLES_CLEAN)

clean: optee-os-clean optee-client-clean $(EXAMPLES_CLEAN)
