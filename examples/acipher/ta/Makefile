OPTEE_DIR ?= ../../../optee
OPTEE_OS_DIR ?= $(OPTEE_DIR)/optee_os
UUID ?= $(shell cat "../uuid.txt")

ARCH ?= aarch64

OPTEE_DIR ?= ../../../optee

ifeq ($(ARCH), arm)
	TA_SIGN_KEY ?= $(OPTEE_OS_DIR)/out/arm/export-ta_arm32/keys/default_ta.pem
	SIGN := $(OPTEE_OS_DIR)/out/arm/export-ta_arm32/scripts/sign.py
	OPTEE_BIN := $(OPTEE_DIR)/toolchains/aarch32/bin
	OBJCOPY := $(OPTEE_BIN)/arm-linux-gnueabihf-objcopy
	TARGET := arm-unknown-optee-trustzone
else
	TA_SIGN_KEY ?= $(OPTEE_OS_DIR)/out/arm/export-ta_arm64/keys/default_ta.pem
	SIGN := $(OPTEE_OS_DIR)/out/arm/export-ta_arm64/scripts/sign.py
	OPTEE_BIN := $(OPTEE_DIR)/toolchains/$(ARCH)/bin
	OBJCOPY := $(OPTEE_BIN)/aarch64-linux-gnu-objcopy
	TARGET := aarch64-unknown-optee-trustzone
endif

OUT_DIR := $(CURDIR)/target/$(TARGET)/release

all: ta strip sign

ta:
	@xargo build --target $(TARGET) --release --verbose

strip:
	@$(OBJCOPY) --strip-unneeded $(OUT_DIR)/ta $(OUT_DIR)/stripped_ta

sign:
	@$(SIGN) --uuid $(UUID) --key $(TA_SIGN_KEY) --in $(OUT_DIR)/stripped_ta --out $(OUT_DIR)/$(UUID).ta
	@echo "SIGN =>  ${UUID}"

clean:
	@xargo clean
