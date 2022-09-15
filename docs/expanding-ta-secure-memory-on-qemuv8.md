---
permalink: /trustzone-sdk-docs/expanding-ta-secure-memory-on-qemuv8.md
---

# Expanding TA Secure Memory on QEMUv8

Since some Rust examples such as `tls_server-rs` and `tls_client-rs`  require
larger TA memory (about 18M heap), we've expanded TA secure memory on OP-TEE
QEMUv8 platform. On QEMUv8 platform it supports 7M TA memory originally, after
expanding it supports 27M TA memory at most.

We modified the firmware and configuration of QEMU, ATF and OPTEE. You can
download the pre-built image from
https://nightlies.apache.org/teaclave/teaclave-trustzone-sdk/ or patch the code
and build the images by yourself.

The modifications are:

1. QEMU patch in `optee-repo/qemu`:

```
diff --git a/hw/arm/virt.c b/hw/arm/virt.c
index d2e5ecd..e1070a0 100644
--- a/hw/arm/virt.c
+++ b/hw/arm/virt.c
@@ -157,7 +157,7 @@ static const MemMapEntry base_memmap[] = {
     [VIRT_MMIO] =               { 0x0a000000, 0x00000200 },
     /* ...repeating for a total of NUM_VIRTIO_TRANSPORTS, each of that size */
     [VIRT_PLATFORM_BUS] =       { 0x0c000000, 0x02000000 },
-    [VIRT_SECURE_MEM] =         { 0x0e000000, 0x01000000 },
+    [VIRT_SECURE_MEM] =         { 0x0e000000, 0x02000000 },
     [VIRT_PCIE_MMIO] =          { 0x10000000, 0x2eff0000 },
     [VIRT_PCIE_PIO] =           { 0x3eff0000, 0x00010000 },
     [VIRT_PCIE_ECAM] =          { 0x3f000000, 0x01000000 },
```

2. ATF patch in `optee-repo/trusted-firmware-a`:

```
diff --git a/plat/qemu/qemu/include/platform_def.h b/plat/qemu/qemu/include/platform_def.h
index c02eff9..ded0660 100644
--- a/plat/qemu/qemu/include/platform_def.h
+++ b/plat/qemu/qemu/include/platform_def.h
@@ -87,7 +87,7 @@
 #define SEC_SRAM_SIZE                  0x00060000

 #define SEC_DRAM_BASE                  0x0e100000
-#define SEC_DRAM_SIZE                  0x00f00000
+#define SEC_DRAM_SIZE                  0x01f00000

 #define SECURE_GPIO_BASE               0x090b0000
 #define SECURE_GPIO_SIZE               0x00001000
```

3. Add configurations in `optee-repo/optee_os`:

```
diff --git a/mk/config.mk b/mk/config.mk
index f2822df..8148cc5 100644
--- a/mk/config.mk
+++ b/mk/config.mk
@@ -904,3 +904,7 @@ CFG_DRIVERS_TPM2_MMIO ?= n
 ifeq ($(CFG_CORE_TPM_EVENT_LOG),y)
 CFG_CORE_TCG_PROVIDER ?= $(CFG_DRIVERS_TPM2)
 endif
+
+# expand TA secure memory
+CFG_TZDRAM_START = 0x0e100000
+CFG_TZDRAM_SIZE = 0x01f00000
```

4. Patch for OP-TEE core pagetable:

```
diff --git a/core/include/mm/pgt_cache.h b/core/include/mm/pgt_cache.h
index 0e72e17..28c58ad 100644
--- a/core/include/mm/pgt_cache.h
+++ b/core/include/mm/pgt_cache.h
@@ -45,9 +45,9 @@ struct pgt {
 #if CFG_NUM_THREADS < 2
 #define PGT_CACHE_SIZE 4
 #elif (CFG_NUM_THREADS == 2 && !defined(CFG_WITH_LPAE))
-#define PGT_CACHE_SIZE 8
+#define PGT_CACHE_SIZE 32
 #else
-#define PGT_CACHE_SIZE ROUNDUP(CFG_NUM_THREADS * 2, PGT_NUM_PGT_PER_PAGE)
+#define PGT_CACHE_SIZE 32
 #endif
```

Finally, build images:

```
$ cd optee-repo/build
$ make
```
