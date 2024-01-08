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

ifeq ($(O),)
out-dir := $(CURDIR)/out
else
out-dir := $(O)
endif

bindir ?= /usr/bin
libdir ?= /usr/lib

ifneq ($V,1)
	q := @
	echo := @echo
else
	q :=
	echo := @:
endif
# export 'q', used by sub-makefiles.
export q

EXAMPLES = $(wildcard examples/*)
EXAMPLES_CLEAN  = $(EXAMPLES:%=%-clean)

TARGET ?= aarch64-unknown-linux-gnu
CROSS_COMPILE ?= aarch64-linux-gnu-

# If _HOST or _TA specific compiler/target are not specified, then use common
# compiler/target for both
CROSS_COMPILE_HOST ?= $(CROSS_COMPILE)
CROSS_COMPILE_TA ?= $(CROSS_COMPILE)
TARGET_HOST ?= $(TARGET)
TARGET_TA ?= $(TARGET)

.PHONY: all examples $(EXAMPLES) install clean
ifneq ($(wildcard $(TA_DEV_KIT_DIR)/host_include/conf.mk),)
all: examples
else
all:
	$(q)echo "TA_DEV_KIT_DIR is not correctly defined" && false
endif

examples: $(EXAMPLES)
$(EXAMPLES):
	$(q)make -C $@ TARGET_HOST=$(TARGET_HOST) \
		TARGET_TA=$(TARGET_TA) \
		CROSS_COMPILE_HOST=$(CROSS_COMPILE_HOST) \
		CROSS_COMPILE_TA=$(CROSS_COMPILE_TA) \
		TA_DEV_KIT_DIR=$(TA_DEV_KIT_DIR) \
		OPTEE_CLIENT_EXPORT=$(OPTEE_CLIENT_EXPORT)

install: examples
	$(echo) '  INSTALL ${out-dir}/lib/optee_armtz'
	$(q)mkdir -p ${out-dir}/lib/optee_armtz
	$(q)find examples/*/ta/target/$(TARGET_TA)/ -name *.ta -exec cp {} ${out-dir}/lib/optee_armtz \;
	$(echo) '  INSTALL ${out-dir}${bindir}'
	$(q)mkdir -p ${out-dir}${bindir}
	$(q)cp examples/*/host/target/$(TARGET_HOST)/release/*-rs ${out-dir}${bindir}
	$(echo) '  INSTALL ${out-dir}${libdir}/tee-supplicant/plugins/'
	$(q)mkdir -p ${out-dir}${libdir}/tee-supplicant/plugins/
	$(q)find examples/*/plugin/target/$(TARGET_HOST)/ -name *.plugin.so -exec cp {} ${out-dir}${libdir}/tee-supplicant/plugins/ \;

examples-clean: $(EXAMPLES_CLEAN) out-clean
$(EXAMPLES_CLEAN):
	$(q)make -C $(@:-clean=) clean

out-clean:
	rm -rf out

clean: $(EXAMPLES_CLEAN) out-clean
