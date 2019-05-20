#!/bin/bash
set -xe

rm -rf screenlog.0
rm -rf optee-qemuv8-3.4.0
rm -rf shared

curl http://mesalock-linux.org/assets/optee-qemuv8-3.4.0.tar.gz | tar zxv
mkdir shared
cp ../examples/hello_world/ta/target/aarch64-unknown-optee-trustzone/debug/*.ta shared
cp ../examples/hello_world/host/target/aarch64-unknown-linux-gnu/debug/hello_world shared

screen -L -d -m -S qemu_screen ./optee-qemuv8.sh
sleep 20
screen -S qemu_screen -p 0 -X stuff "root\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "mkdir shared && mount -t 9p -o trans=virtio host shared && cd shared\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "cp *.ta /lib/optee_armtz/\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "./hello_world\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "^C"
sleep 5

{
	grep -q "Success" screenlog.0 &&
	grep -q "[+] Get REE time (second: [0-9]*, millisecond: [0-9]*)." /tmp/serial.log &&
	grep -q "[+] Now wait 5 seconds in TEE ..." /tmp/serial.log &&
	grep -q "[+] Get system time (second: [0-9]*, millisecond: [0-9]*)." /tmp/serial.log &&
	grep -q "[+] After set the TA time 5 seconds ahead of system time, new TA time (second: [0-9]*, millisecond: [0-9]*)." /tmp/serial.log
} || {
	cat -v screenlog.0
	cat -v /tmp/serial.log
}

rm -rf screenlog.0
rm -rf optee-qemuv8-3.4.0
rm -rf shared
