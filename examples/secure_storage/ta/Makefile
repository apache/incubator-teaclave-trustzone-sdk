OPTEE_DIR ?= ../../../optee
OPTEE_OS_DIR ?= $(OPTEE_DIR)/optee_os
OPTEE_ARCH ?= aarch64
UUID ?= f4e750bb-1437-4fbf-8785-8d3580c34994
TA_SIGN_KEY ?= $(OPTEE_DIR)/optee_os/out/arm/export-ta_arm64/keys/default_ta.pem
OPTEE_OS_TA_DEV_KIT_DIR ?= $(OPTEE_OS_DIR)/out/arm/export-ta_arm64

SRC_DIR := $(CURDIR)/src
OUT_DIR := $(CURDIR)/target/aarch64-unknown-optee-trustzone/debug

OPTEE_TOOLCHAINS := $(OPTEE_DIR)/toolchains
CC := $(OPTEE_TOOLCHAINS)/aarch64/bin/aarch64-linux-gnu-gcc
LD := $(OPTEE_TOOLCHAINS)/aarch64/bin/aarch64-linux-gnu-ld.bfd
OBJCOPY := $(OPTEE_TOOLCHAINS)/aarch64/bin/aarch64-linux-gnu-objcopy
SIGN := $(OPTEE_DIR)/optee_os/out/arm/export-ta_arm64/scripts/sign.py

CFLAGS := -Wall -Os -fpie -std=gnu99 -nostdinc -DCFG_TEE_TA_LOG_LEVEL=4 \
          -DARM64=1 -D__LP64__=1 -DTRACE_LEVEL=4
INCLUDES := -isystem $(OPTEE_TOOLCHAINS)/aarch64/lib/gcc/aarch64-linux-gnu/8.2.1/include \
            -I$(SRC_DIR) -I$(OPTEE_DIR)/optee_os/out/arm/export-ta_arm64/include
LDFLAGS := -pie -Tta.lds -Map=$(OUT_DIR)/$(UUID).map --sort-section=alignment \
           -L$(OUT_DIR) -L$(OPTEE_OS_TA_DEV_KIT_DIR)/lib   \
           --start-group -lmpa -lutils -lutee -lta --end-group

all: talib taheader talink tastrip tasign

talib:
	xargo build --target aarch64-unknown-optee-trustzone

taheader:
	$(CC) $(CFLAGS) $(INCLUDES) -c $(OPTEE_OS_TA_DEV_KIT_DIR)/src/user_ta_header.c -o $(OUT_DIR)/user_ta_header.o

talink:
	$(LD) $(OUT_DIR)/user_ta_header.o $(LDFLAGS) -o $(OUT_DIR)/$(UUID).elf

tastrip:
	$(OBJCOPY) --strip-unneeded $(OUT_DIR)/$(UUID).elf $(OUT_DIR)/$(UUID).stripped.elf

tasign:
	$(SIGN) --uuid $(UUID) --key $(TA_SIGN_KEY) --in $(OUT_DIR)/$(UUID).stripped.elf --out $(OUT_DIR)/$(UUID).ta

clean:
	xargo clean
