boot: build
	mv target/x86_64-unknown-none/debug/bootimage-Samsara.bin ./Samsara.bin
	qemu-system-x86_64 -drive format=raw,file=./Samsara.bin
reboot:
	qemu-system-x86_64 -drive format=raw,file=./Samsara.bin
build:
	cargo bootimage
clean:
	rm -r ./target
	rm ./Samsara.bin
