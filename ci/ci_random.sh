#!/bin/bash
set -xe

rm -rf screenlog.0
rm -rf optee-qemuv8-3.4.0
rm -rf shared

curl http://mesalock-linux.org/assets/optee-qemuv8-3.4.0.tar.gz | tar zxv
mkdir shared
cp ../examples/random/ta/target/aarch64-unknown-optee-trustzone/debug/*.ta shared
cp ../examples/random/host/target/aarch64-unknown-linux-gnu/debug/random shared

screen -L -d -m -S qemu_screen ./optee-qemuv8.sh
sleep 20
screen -S qemu_screen -p 0 -X stuff "root\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "mkdir shared && mount -t 9p -o trans=virtio host shared && cd shared\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "cp *.ta /lib/optee_armtz/\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "./random\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "^C"
sleep 5
cat -v screenlog.0
cat -v /tmp/serial.log
grep -q "Invoking TA to generate random UUID..." screenlog.0
grep -q "Generate random UUID: [a-z0-9]*-[a-z0-9]*-[a-z0-9]*-[a-z0-9]*" screenlog.0
grep -q "Success" screenlog.0

rm -rf screenlog.0
rm -rf optee-qemuv8-3.4.0
rm -rf shared
