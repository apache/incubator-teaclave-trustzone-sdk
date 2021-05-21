#!/bin/bash
set -xe

rm -rf screenlog.0
rm -rf optee-qemuv8-3.13.0
rm -rf shared

curl http://mesalock-linux.org/assets/optee-qemuv8-3.13.0.tar.gz | tar zxv
mkdir shared
cp ../examples/acipher/ta/target/aarch64-unknown-optee-trustzone/release/*.ta shared
cp ../examples/acipher/host/target/aarch64-unknown-linux-gnu/release/acipher shared

screen -L -d -m -S qemu_screen ./optee-qemuv8.sh
sleep 30
screen -S qemu_screen -p 0 -X stuff "root\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "mkdir shared && mount -t 9p -o trans=virtio host shared && cd shared\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "cp *.ta /lib/optee_armtz/\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "./acipher 256 teststring\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "^C"
sleep 5

{
	grep -q "Success encrypt input text \".*\" as [0-9]* bytes cipher text:" screenlog.0 &&
	grep -q "Success decrypt the above ciphertext as [0-9]* bytes plain text:" screenlog.0	
} || {
	cat -v screenlog.0
	cat -v /tmp/serial.log
        false
}

rm -rf screenlog.0
rm -rf optee-qemuv8-3.13.0
rm -rf shared
