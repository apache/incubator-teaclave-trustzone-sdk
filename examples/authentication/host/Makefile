NAME := authentication
ARCH ?= aarch64

OPTEE_DIR ?= ../../../optee

ifeq ($(ARCH), arm)
	OPTEE_BIN := $(OPTEE_DIR)/toolchains/aarch32/bin
	OBJCOPY := $(OPTEE_BIN)/arm-linux-gnueabihf-objcopy
	TARGET := arm-unknown-linux-gnueabihf
else
	OPTEE_BIN := $(OPTEE_DIR)/toolchains/$(ARCH)/bin
	OBJCOPY := $(OPTEE_BIN)/aarch64-linux-gnu-objcopy
	TARGET := aarch64-unknown-linux-gnu
endif

OUT_DIR := $(CURDIR)/target/$(TARGET)/release


all: host strip

host:
	@cargo build --target $(TARGET) --release

strip:
	@$(OBJCOPY) --strip-unneeded $(OUT_DIR)/$(NAME) $(OUT_DIR)/$(NAME)

clean:
	@cargo clean
