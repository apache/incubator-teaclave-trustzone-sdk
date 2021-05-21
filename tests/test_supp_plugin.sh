#!/bin/bash
set -xe

rm -rf screenlog.0
rm -rf optee-qemuv8-3.13.0
rm -rf shared

curl http://mesalock-linux.org/assets/optee-qemuv8-3.13.0.tar.gz | tar zxv
mkdir shared
cp ../examples/supp_plugin/ta/target/aarch64-unknown-optee-trustzone/release/*.ta shared
cp ../examples/supp_plugin/host/target/aarch64-unknown-linux-gnu/release/supp_plugin shared
cp ../examples/supp_plugin/plugin/target/aarch64-unknown-linux-gnu/release/*.plugin.so shared

screen -L -d -m -S qemu_screen ./optee-qemuv8.sh
sleep 30
screen -S qemu_screen -p 0 -X stuff "root\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "mkdir shared && mount -t 9p -o trans=virtio host shared && cd shared\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "cp *.ta /lib/optee_armtz/\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "cp *.plugin.so /usr/lib/tee-supplicant/plugins/\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "kill \$(pidof tee-supplicant)\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "/usr/sbin/tee-supplicant &\n\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "./supp_plugin\n"
sleep 5
screen -S qemu_screen -p 0 -X stuff "^C"
sleep 5

{
	grep -q "send value" screenlog.0 &&
	grep -q "invoke" screenlog.0 &&
	grep -q "receive value" screenlog.0 &&
	grep -q "invoke commmand finished" screenlog.0 &&
	grep -q "Success" screenlog.0
} || {
	cat -v screenlog.0
	cat -v /tmp/serial.log
	false
}

rm -rf screenlog.0
rm -rf optee-qemuv8-3.13.0
rm -rf shared
