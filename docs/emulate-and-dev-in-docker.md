---
permalink: /trustzone-sdk-docs/dev-and-emulate-in-docker.md
---

# 🚀 Quick Start For QEMU Emulation

This guide walks you through setting up the Teaclave TrustZone SDK and running the Hello World example in a fully emulated environment using QEMU. The workflow uses four terminals to provide real-time monitoring of both the guest VM and Trusted Application output.

## 📋 Prerequisites

- Docker installed on your system
- Git for cloning the repository

## 🛠️ Setup and Build

### 1. Pull Development Docker Image

**Terminal A** (Main development terminal):
```bash
# Pull the pre-built development environment
$ docker pull teaclave-trustzone-emulator-nostd-optee-4.5.0-expand-memory:latest

# Clone the repository
$ git clone https://github.com/apache/incubator-teaclave-trustzone-sdk.git && \
  cd incubator-teaclave-trustzone-sdk

# Launch the development container
$ docker run -it --rm \
  --name teaclave_dev_env \
  -v $(pwd):/root/teaclave_sdk_src \
  teaclave-trustzone-emulator-nostd-optee-4.5.0-expand-memory:latest
```

### 2. Build the Hello World Example

**Still in Terminal A** (inside the Docker container):
```bash
# Build the Hello World example (both CA and TA)
$ make -C examples/hello_world-rs/

# Sync the built artifacts to share with the emulator 
# We suggest following a similar template to add emulate target for your own project
$ make -C examples/hello_world-rs/ emulate
```

> **💡 Pro Tip:** Internally, we've prepared a built-in command `sync_to_emulator` to abstract the complexity of syncing between emulator and built targets. Run `sync_to_emulator -h` for more details.

## 🎭 Multi-Terminal Execution

The emulation workflow requires **three additional terminals** to monitor different aspects of the system in order to differentiate between the normal world, secure world of the emulator guest, and QEMU itself:

- **Terminal B**: 🖥️ **Normal World Listener** - Provides access to the guest VM shell
- **Terminal C**: 🔒 **Secure World Listener** - Monitors Trusted Application output logs  
- **Terminal D**: 🚀 **QEMU Control** - Controls the QEMU emulator

### 3. Setup Monitoring Terminals

We have provided built-in commands in the development environment. The executable path `/opt/teaclave/bin/` has been exported in the default user's profile. Feel free to use either `bash -l` or the full path when executing with `docker exec`.

**Terminal B** (Guest VM Shell):
```bash
# Connect to the guest VM shell for running commands inside the emulated environment
$ docker exec -it teaclave_dev_env bash -l -c listen_on_guest_vm_shell

# Alternative: Use full path
$ docker exec -it teaclave_dev_env /opt/teaclave/bin/listen_on_guest_vm_shell
```

**Terminal C** (TA Output Monitor):
```bash
# Monitor Trusted Application output logs in real-time
$ docker exec -it teaclave_dev_env bash -l -c listen_on_ta_output

# Alternative: Use full path  
$ docker exec -it teaclave_dev_env /opt/teaclave/bin/listen_on_ta_output
```

### 4. Start the Emulation

After the listeners are set up, we can start the QEMU emulator and expect to get output in Terminal B and C.

**Terminal D** (QEMU Control):
```bash
# Launch QEMU emulator with debug output and connect to monitoring ports
$ docker exec -it teaclave_dev_env bash -l -c "DEBUG=1 start_qemuv8"
```

> ⏳ **Wait for the QEMU environment to fully boot...** You should see boot messages in Terminal D and the guest VM shell prompt in Terminal B.

### 5. Run the Hello World Example

After QEMU in Terminal D successfully launches, go back to Terminal B for the shell in the guest VM normal world.

**Terminal B** (Inside Guest VM):
```bash
# Execute the Hello World Client Application
$ ./host/hello_world-rs
```

## 🎉 Expected Results

- **Terminal A**: 📦 Development environment - keeps container active for building and syncing
- **Terminal B**: 🖥️ Normal world - Displays the Hello World CA output and provides guest VM shell access
- **Terminal C**: 🔒 Secure world - Shows detailed TA execution logs, debug information, and secure world messages  
- **Terminal D**: 🚀 QEMU control - Shows QEMU boot sequence, system logs, and emulator status

### Development Environment Details

The setup scripts and built-in commands can be found in `/opt/teaclave/`. Please refer to the Dockerfile in the SDK source repository for more information about how we set up the development environment.