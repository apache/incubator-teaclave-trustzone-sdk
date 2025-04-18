# Patches to Customize Our Test QEMU Image

This folder contains patch files used to build our custom QEMU test image.

## Patches for IPV6 Support

The official QEMUv8 configuration in OP-TEE does not currently support IPv6,
which is required for our IPv6 tests.

We temporarily use a patch to enable IPv6 support.
This patch can be removed once Issue [#174](https://github.com/apache/incubator-teaclave-trustzone-sdk/issues/174)
is resolved.

Relevant Patch File:

1. optee-build_ipv6_support.patch: Enables IPv6 support in the Linux kernel.

## Patches for Expand Memory

Some of our tests require more Trusted Application (TA) memory than the default
OP-TEE configuration provides.

Relevant Patch File:

1. qemu-qemu_expand_secure_memory.patch: Increases the size of VIRT_SECURE_MEM
    in QEMU.
2. arm-atf_expand_secure_memory.patch: Updates ARM Trusted Firmware definitions
    to match the QEMU memory expansion.
3. optee-build_expand_memory.patch: Updates build configurations to reflect the
    expanded memory setup.
