NAME := aes
OPTEE_DIR ?= ../../../optee
OUT_DIR := $(CURDIR)/target/aarch64-unknown-linux-gnu/release
OPTEE_BIN := $(OPTEE_DIR)/toolchains/aarch64/bin
OBJCOPY := $(OPTEE_BIN)/aarch64-linux-gnu-objcopy

all: host strip

host:
	@cargo build --target aarch64-unknown-linux-gnu --release

strip:
	@$(OBJCOPY) --strip-unneeded $(OUT_DIR)/$(NAME) $(OUT_DIR)/$(NAME)

clean:
	@cargo clean
