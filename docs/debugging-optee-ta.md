---
permalink: /trustzone-sdk-docs/debugging-optee-ta.md
---

# Debugging OP-TEE TA

When developing applications, it is inevitable that there will be a need for
debugging. This tutorial introduces how to configure the debug environment in
OP-TEE enabled QEMU environment. You may also check
[OP-TEE documentation](https://optee.readthedocs.io/en/latest/building/devices/qemu.html)
for more information about running QEMU for Arm v8.

To debug TEE core running QEMU with GDB, it is necessary to disable TEE ASLR with
`CFG_CORE_ASLR ?= n` in `OP-TEE/optee_os/mk/config.mk`. Note that you need to
recompile QEMU with `make run` again. You can also choose to add the compilation
information directly at compile time.
```sh
$ make run CFG_CORE_ASLR=n
```

Since we will debug the TA remotely with a `gdb` server, please also add the
`GDBSERVER=y` flag when compiling QEMU.

To debug a TA, you need to first start a gdb on the host machine. Then run
`target remote :1234` to connect to the remote QEMU GDB server.

```sh
$ ./path/to/qemu-v8-project/out-br/host/bin/aarch64-buildroot-linux-gnu-gdb
(gdb) target remote :1234
Remote debugging using :1234
warning: No executable has been specified and target does not support
determining executable automatically.  Try using the "file" command.
0xffffb30b00ea12b4 in ?? ()
```
Next, in the GDB console, load the symbol table of the TEE core library.

```sh
(gdb) symbol-file /path/to/qemu-v8-project/optee_os/out/arm/core/tee.elf
```
Taking `hello_world-rs` as an example, you can get the start address of the text
section from the log in the secure world console, which is 0x40014000.

```sh
D/LD:  ldelf:168 ELF (133af0ca-bdab-11eb-9130-43bf7873bf67) at 0x40014000
```

Then, you can load symbols from TA file (in debug build) to the address.
```sh
(gdb) add-symbol-file /path/to/examples/hello_world-rs/ta/target/aarch64-unknown-linux-gnu/debug/ta 0x40014000
```
Now, you can add breakpoints according to your own needs in the corresponding
functions or addresses.
```sh
(gdb) b invoke_command
Breakpoint 2 at 0xe11bb08: invoke_command. (6 locations)
```
Last, initiate the boot. You can execute `hello_world-rs` in the normal world
console, and will see that the breakpoint we set was hit.
```sh
(gdb) c
Continuing.
[Switching to Thread 1.2]

Thread 2 hit Breakpoint 2, ta::invoke_command (cmd_id=0, params=0x4010ff00) at src/main.rs:50
50	    trace_println!("[+] TA invoke command");
```
