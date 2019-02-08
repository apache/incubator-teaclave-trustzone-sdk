FILE_NAME = ta
OPTEE_PATH = $(CURDIR)/../../../optee
OPTEE_BIN = $(OPTEE_PATH)/toolchains/aarch64/bin
DIR = ${CURDIR}
SRCDIR = ${DIR}/src
OUTDIR = ${DIR}/target/aarch64-unknown-optee-trustzone/debug
#update UUID for different TAs
UUID = 8abcf200-2450-11e4-abe2-0002a5d5c51b

CC = $(OPTEE_BIN)/aarch64-linux-gnu-gcc
LD = $(OPTEE_BIN)/aarch64-linux-gnu-ld.bfd
OBJCOPY = $(OPTEE_BIN)/aarch64-linux-gnu-objcopy
SIGN = $(OPTEE_PATH)/optee_os/out/arm/export-ta_arm64/scripts/sign.py

HEADERFLAG = -Werror -fdiagnostics-show-option -Wall -Wcast-align -Werror-implicit-function-declaration -Wextra -Wfloat-equal -Wformat-nonliteral -Wformat-security -Wformat=2 -Winit-self -Wmissing-declarations -Wmissing-format-attribute -Wmissing-include-dirs -Wmissing-noreturn -Wmissing-prototypes -Wnested-externs -Wpointer-arith -Wshadow -Wstrict-prototypes -Wswitch-default -Wwrite-strings -Wno-missing-field-initializers -Wno-format-zero-length -Waggregate-return -Wredundant-decls -Wold-style-definition -Wstrict-aliasing=2 -Wundef -pedantic -Wdeclaration-after-statement -Wno-error=cast-align -Os -g3 -fpie -MD -MF

all: talib taheader talink tastrip tasign

talib:
	RUST_TARGET_PATH=$(DIR)/../../../ xargo build --target aarch64-unknown-optee-trustzone

taheader:
	$(CC) -std=gnu99 $(HEADERFLAG) ${SRCDIR}/.user_ta_header.o.d -MT user_ta_header.o -nostdinc -isystem $(OPTEE_BIN)/../lib/gcc/aarch64-linux-gnu/8.2.1/include -DCFG_TEE_TA_LOG_LEVEL=4 -DARM64=1 -D__LP64__=1 -DTRACE_LEVEL=4 -I$(SRCDIR) -I$(OPTEE_PATH)/optee_os/out/arm/export-ta_arm64/include -c $(OPTEE_PATH)/optee_os/out/arm/export-ta_arm64/src/user_ta_header.c -o $(OUTDIR)/user_ta_header.o

talink:
	$(LD) -pie -T $(SRCDIR)/ta.lds -Map=$(OUTDIR)/$(UUID).map --sort-section=alignment -L $(OUTDIR) $(OUTDIR)/user_ta_header.o -L$(OPTEE_PATH)/optee_os/out/arm/export-ta_arm64/lib -rpath-link $(OPTEE_PATH)/out-br/target/lib --start-group -lmpa -lutils -lutee -l$(FILE_NAME) --end-group -o $(OUTDIR)/$(UUID).elf

tastrip:
	$(OBJCOPY) --strip-unneeded $(OUTDIR)/$(UUID).elf $(OUTDIR)/$(UUID).stripped.elf

tasign:
	$(SIGN) --uuid $(UUID) --key $(OPTEE_PATH)/build/../optee_os/out/arm/export-ta_arm64/keys/default_ta.pem --in $(OUTDIR)/$(UUID).stripped.elf --out $(OUTDIR)/$(UUID).ta

clean:
	xargo clean
