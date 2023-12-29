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

ifneq ($V,1)
	q := @
	echo := @echo
else
	q :=
	echo := @:
endif

EXAMPLES = $(wildcard examples/*)
EXAMPLES_INSTALL = $(EXAMPLES:%=%-install)
EXAMPLES_CLEAN  = $(EXAMPLES:%=%-clean)

ifneq ($(ARCH), arm)
	CROSS_COMPILE ?= aarch64-linux-gnu-
	TARGET := aarch64-unknown-linux-gnu
else
	CROSS_COMPILE ?= arm-linux-gnueabihf-
	TARGET := arm-unknown-linux-gnueabihf
endif

export ARCH

.PHONY: all
ifneq ($(wildcard $(TA_DEV_KIT_DIR)/host_include/conf.mk),)
all: examples examples-install
else
all:
	$(q)echo "TA_DEV_KIT_DIR is not correctly defined" && false
endif

examples: $(EXAMPLES)
$(EXAMPLES):
	$(q)make -C $@ CROSS_COMPILE=$(CROSS_COMPILE) TA_DEV_KIT_DIR=$(TA_DEV_KIT_DIR) \
		OPTEE_CLIENT_EXPORT=$(OPTEE_CLIENT_EXPORT)

examples-install: $(EXAMPLES_INSTALL)
$(EXAMPLES_INSTALL): examples
	install -D $(@:%-install=%)/host/target/$(TARGET)/release/$(@:examples/%-install=%) -t out/host/
	install -D $(@:%-install=%)/ta/target/$(TARGET)/release/*.ta -t out/ta/
	$(q)if [ -d "$(@:%-install=%)/plugin/target/" ]; then \
		install -D $(@:%-install=%)/plugin/target/$(TARGET)/release/*.plugin.so -t out/plugin/; \
	fi

examples-clean: $(EXAMPLES_CLEAN) out-clean
$(EXAMPLES_CLEAN):
	$(q)make -C $(@:-clean=) clean

out-clean:
	rm -rf out

.PHONY: clean $(EXAMPLES) $(EXAMPLES_CLEAN)

clean: $(EXAMPLES_CLEAN) out-clean
