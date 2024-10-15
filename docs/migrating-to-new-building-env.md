---
permalink: /trustzone-sdk-docs/migrating-to-new-building-env.md
---

## Migration Guide: Moving from `master` to `main` Branch (Post-Oct 2024)

Since the `main` branch (after October 2024) introduces breaking changes 
to the build environment, if users of the legacy `master` branch want to 
keep upstream or use a new version of the Rust toolchain, they will need 
to follow these steps to migrate their TA to the new environment.

### Step 1: Migrate Old Code to the New Environment

To migrate an example project (e.g., `tls_server-rs` in `examples/`), you 
can refer to the following shell script and adjust it for your TA:

```bash
TARGET_EXAMPLE="tls_server-rs"
OLD_ROOT_PATH="/path/to/old/sdk"
NEW_PATH="/path/to/current/sdk"

# Duplicate the hello-world example in the new path as a template
cp -r ${NEW_PATH}/examples/hello_world-rs ${NEW_PATH}/examples/${TARGET_EXAMPLE}

# Remove the source code directory and copy from the old path to the new path
# including: src/ and Cargo.toml in host, ta, proto
(cd ${NEW_PATH}/examples/${TARGET_EXAMPLE}/host && rm -rf src/ Cargo.* && \
cp -r ${OLD_ROOT_PATH}/examples/${TARGET_EXAMPLE}/host/src . && \
cp ${OLD_ROOT_PATH}/examples/${TARGET_EXAMPLE}/host/Cargo.toml .)
(cd ${NEW_PATH}/examples/${TARGET_EXAMPLE}/ta && rm -rf src/ Cargo.* && \
cp -r ${OLD_ROOT_PATH}/examples/${TARGET_EXAMPLE}/ta/src . && \
cp ${OLD_ROOT_PATH}/examples/${TARGET_EXAMPLE}/ta/Cargo.toml .)
(cd ${NEW_PATH}/examples/${TARGET_EXAMPLE}/proto && rm -rf src/ Cargo.* && \
cp -r ${OLD_ROOT_PATH}/examples/${TARGET_EXAMPLE}/proto/src . && \
cp ${OLD_ROOT_PATH}/examples/${TARGET_EXAMPLE}/proto/Cargo.toml .)

# Copy the UUID file from the old path to the new path
cp ${OLD_ROOT_PATH}/examples/${TARGET_EXAMPLE}/uuid.txt \
${NEW_PATH}/examples/${TARGET_EXAMPLE}/uuid.txt

# Update binary names in host/Cargo.toml and host/Makefile
sed -i "s/hello_world-rs/${TARGET_EXAMPLE}/g" \
${NEW_PATH}/examples/${TARGET_EXAMPLE}/host/Cargo.toml
sed -i "s/hello_world-rs/${TARGET_EXAMPLE}/g" \
${NEW_PATH}/examples/${TARGET_EXAMPLE}/host/Makefile
```

### Step 2: Update `Cargo.toml`

You may need to update your `Cargo.toml` to use newer versions of crates. 
The Rust toolchain for the current build environment can be found in 
`rust-toolchain.toml`.

### Step 3: Build and Fix Errors

After updating, run the build process. Some errors might occur due to the 
crate version upgrades—fix them as necessary.
