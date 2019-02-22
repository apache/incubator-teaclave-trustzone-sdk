OPTEE_DIR ?= ../../../optee
OPTEE_OS_DIR ?= $(OPTEE_DIR)/optee_os
UUID ?= $(shell cat "../uuid.txt")
TA_SIGN_KEY ?= $(OPTEE_DIR)/optee_os/out/arm/export-ta_arm64/keys/default_ta.pem
OUT_DIR := $(CURDIR)/target/aarch64-unknown-optee-trustzone/debug
SIGN := $(OPTEE_DIR)/optee_os/out/arm/export-ta_arm64/scripts/sign.py

all: ta sign

ta:
	xargo rustc --target aarch64-unknown-optee-trustzone -vv -- -Z print-link-args

sign:
	$(SIGN) --uuid $(UUID) --key $(TA_SIGN_KEY) --in $(OUT_DIR)/ta --out $(OUT_DIR)/$(UUID).ta

clean:
	xargo clean
