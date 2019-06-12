OPTEE_DIR ?= ../../../optee
OPTEE_OS_DIR ?= $(OPTEE_DIR)/optee_os
UUID ?= $(shell cat "../uuid.txt")
TA_SIGN_KEY ?= $(OPTEE_DIR)/optee_os/out/arm/export-ta_arm64/keys/default_ta.pem
OUT_DIR := $(CURDIR)/target/aarch64-unknown-optee-trustzone/release
SIGN := $(OPTEE_DIR)/optee_os/out/arm/export-ta_arm64/scripts/sign.py
OPTEE_BIN := $(OPTEE_DIR)/toolchains/aarch64/bin
OBJCOPY := $(OPTEE_BIN)/aarch64-linux-gnu-objcopy

all: ta strip sign

ta:
	@xargo build --target aarch64-unknown-optee-trustzone --release

strip:
	@$(OBJCOPY) --strip-unneeded $(OUT_DIR)/ta $(OUT_DIR)/stripped_ta

sign:
	@$(SIGN) --uuid $(UUID) --key $(TA_SIGN_KEY) --in $(OUT_DIR)/stripped_ta --out $(OUT_DIR)/$(UUID).ta
	@echo "SIGN =>  ${UUID}"

clean:
	@xargo clean
