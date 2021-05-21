NAME := syslog_plugin
ARCH ?= aarch64
PLUGIN_UUID := `cat ../plugin_uuid.txt`

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


all: host

host:
	@cargo build --target $(TARGET) --release
	mv $(CURDIR)/target/$(TARGET)/release/lib$(NAME).so $(CURDIR)/target/$(TARGET)/release/$(PLUGIN_UUID).plugin.so 
clean:
	@cargo clean
