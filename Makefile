OPTEE_PATH        ?= $(CURDIR)/optee
OPTEE_BUILD_PATH  ?= $(OPTEE_PATH)/build
OPTEE_OS_PATH     ?= $(OPTEE_PATH)/optee_os
OPTEE_CLIENT_PATH ?= $(OPTEE_PATH)/optee_client
VENDOR            ?= qemu_v8.mk

all: toolchains optee-os optee-client

toolchains:
	make -C $(OPTEE_BUILD_PATH) -f $(VENDOR) toolchains

optee-os:
	make -C $(OPTEE_BUILD_PATH) -f $(VENDOR) optee-os

optee-client:
	make -C $(OPTEE_BUILD_PATH) -f $(VENDOR) optee-client

optee-os-clean:
	make -C $(OPTEE_OS_PATH) O=out/arm clean

optee-client-clean:
	make -C $(OPTEE_CLIENT_PATH) clean

.PHONY: clean

clean: optee-os-clean optee-client-clean
