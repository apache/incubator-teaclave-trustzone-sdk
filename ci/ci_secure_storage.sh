#!/bin/bash
set -xe

rm -rf screenlog.0
rm -rf optee-qemuv8-3.4.0
rm -rf shared

curl http://mesalock-linux.org/assets/optee-qemuv8-3.4.0.tar.gz | tar zxv
mkdir shared
cp ../examples/secure_storage/ta/target/aarch64-unknown-optee-trustzone/debug/*.ta shared
cp ../examples/secure_storage/host/target/aarch64-unknown-linux-gnu/debug/secure_storage shared

screen -L -d -m -S qemu_screen ./optee-qemuv8.sh
sleep 20
screen -S qemu_screen -p 0 -X stuff "root\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "mkdir shared && mount -t 9p -o trans=virtio host shared && cd shared\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "cp *.ta /lib/optee_armtz/\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "./secure_storage\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "^C"
sleep 5

{
	grep -q "Test on object \"object#1\"" screenlog.0
	grep -q "\- Create and load object in the TA secure storage" screenlog.0
	grep -q "\- Read back the object" screenlog.0
	grep -q "\- Content read-out correctly" screenlog.0
	grep -q "\- Delete the object" screenlog.0

	grep -q "Test on object \"object#2\"" screenlog.0
	#miss the read correctly output
	grep -Eq "\- Object not found in TA secure storage, create it|\- Object found in TA secure storage, delete it" screenlog.0

	grep -q "We're done, close and release TEE resources" screenlog.0
} || {
        cat -v screenlog.0
        cat -v /tmp/serial.log
}

rm -rf screenlog.0
rm -rf optee-qemuv8-3.4.0
rm -rf shared
