---
permalink: /trustzone-sdk-docs/debug-optee-ta.md
---

# Debug OP-TEE TA 

When developing applications, it is inevitable that there will be a need for debugging. This tutorial introduces how to configure debug environment in OP-TEE enabled QEMU environment. You may also check [OP-TEE documentation](https://optee.readthedocs.io/en/latest/building/devices/qemu.html) for more information about running QEMU for Arm v8.

To debug TEE core running QEMU with GDB, it is necessary to disable TEE ASLR with `CFG_CORE_ASLR ?= n` in `OP-TEE/optee_os/mk/config.mk`. Note that then recompile with `make run`. You can also choose to add compilation information directly at compile time.
```sh
$ make run CFG_CORE_ASLR=n
```

Since the program is debugged on your PC, while the program being debugged runs in the QEMU environment, this should be added at compile time: `GDBSERVER=y`. 

After starting GDB, executing `target remote :1234` in the normal world console to connect to QEMU GDB server.

```sh
$ ./path/to/qemu-v8-project/out-br/host/bin/aarch64-buildroot-linux-gnu-gdb
(gdb) target remote :1234
Remote debugging using :1234
warning: No executable has been specified and target does not support
determining executable automatically.  Try using the "file" command.
0xffffb30b00ea12b4 in ?? ()
```
Next, in the GDB console, load the symbol table for TEE.

```sh
(gdb) symbol-file /path/to/qemu-v8-project/optee_os/out/arm/core/tee.elf
```
Taking `hello_world-rs` as an example, you can know as prompted in the secure world console, the start address of TA text is 0x40014000.

```sh
D/LD:  ldelf:168 ELF (133af0ca-bdab-11eb-9130-43bf7873bf67) at 0x40014000
```

Then, you can load TA symbol table from the address.
```sh
(gdb) add-symbol-file /path/to/examples/hello_world-rs/ta/target/aarch64-unknown-optee-trustzone/debug/ta 0x40014000
```
Now, you can add breakpoints according to your own needs in the corresponding functions or addresses.
```sh
(gdb) b invoke_command
Breakpoint 2 at 0xe11bb08: invoke_command. (6 locations)
```
Last, initiate the boot. You can execute `hello_world-rs` in the normal world console, and will see that the breakpoint we set was hit.
```sh
(gdb) c
Continuing.
[Switching to Thread 1.2]

Thread 2 hit Breakpoint 2, ta::invoke_command (cmd_id=0, params=0x4010ff00) at src/main.rs:50
50	    trace_println!("[+] TA invoke command");
```



