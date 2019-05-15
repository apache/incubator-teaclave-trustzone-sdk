#!/bin/bash
set -xe

rm -rf screenlog.0
rm -rf optee-qemuv8-3.4.0
rm -rf shared

curl http://mesalock-linux.org/assets/optee-qemuv8-3.4.0.tar.gz | tar zxv
mkdir shared
cp ../examples/message_passing_interface/ta/target/aarch64-unknown-optee-trustzone/release/*.ta shared
cp ../examples/message_passing_interface/host/target/aarch64-unknown-linux-gnu/release/message_passing_interface shared

screen -L -d -m -S qemu_screen ./optee-qemuv8.sh
sleep 20
screen -S qemu_screen -p 0 -X stuff "root\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "mkdir shared && mount -t 9p -o trans=virtio host shared && cd shared\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "cp *.ta /lib/optee_armtz/\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "./message_passing_interface\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "^C"
sleep 5

{
	grep -q "Hello, World" screenlog.0
} || {
	cat -v screenlog.0
	cat -v /tmp/serial.log
	false
}

rm -rf screenlog.0
rm -rf optee-qemuv8-3.4.0
rm -rf shared
