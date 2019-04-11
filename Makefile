OPTEE_PATH        ?= $(CURDIR)/optee
OPTEE_BUILD_PATH  ?= $(OPTEE_PATH)/build
OPTEE_OS_PATH     ?= $(OPTEE_PATH)/optee_os
OPTEE_CLIENT_PATH ?= $(OPTEE_PATH)/optee_client
VENDOR            ?= qemu_v8.mk

EXAMPLES = $(wildcard examples/*)
EXAMPLES_INSTALL = $(EXAMPLES:%=%-install)
EXAMPLES_CLEAN  = $(EXAMPLES:%=%-clean)

all: toolchains optee-os optee-client examples
optee: toolchains optee-os optee-client

toolchains:
	make -C $(OPTEE_BUILD_PATH) -f $(VENDOR) toolchains

optee-os:
	make -C $(OPTEE_BUILD_PATH) -f $(VENDOR) optee-os

optee-client:
	make -C $(OPTEE_BUILD_PATH) -f $(VENDOR) optee-client-common

examples: $(EXAMPLES)
$(EXAMPLES):
	make -C $@

examples-install: $(EXAMPLES_INSTALL)
$(EXAMPLES_INSTALL):
	install -D $(@:%-install=%)/host/target/aarch64-unknown-linux-gnu/debug/$(@:examples/%-install=%) -t out/host/
	install -D $(@:%-install=%)/ta/target/aarch64-unknown-optee-trustzone/debug/*.ta -t out/ta/

optee-os-clean:
	make -C $(OPTEE_OS_PATH) O=out/arm clean

optee-client-clean:
	make -C $(OPTEE_BUILD_PATH) -f $(VENDOR) optee-client-clean-common

examples-clean: $(EXAMPLES_CLEAN) out-clean
$(EXAMPLES_CLEAN):
	make -C $(@:-clean=) clean

out-clean:
	rm -rf out

.PHONY: clean optee-os-clean optee-client-clean $(EXAMPLES) $(EXAMPLES_CLEAN)

clean: optee-os-clean optee-client-clean $(EXAMPLES_CLEAN)
