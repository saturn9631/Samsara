boot: build
	#mv target/x86_64-unknown-none/debug/bootimage-Samsara.bin ./Samsara.bin
	#qemu-system-x86_64 -drive format=raw,file=./Samsara.bin
	cp ./Samsara.bin ./iso/boot/Samsara.bin
	grub-rescue -o Samsara.iso iso
	qemu-system-i386 -cdrom Samsara.iso
reboot:
	#qemu-system-x86_64 -drive format=raw,file=./Samsara.bin
	cp ./Samsara.bin ./iso/boot/Samsara.bin
	grub-rescue -o Samsara.iso iso
	qemu-system-i386 -cdrom Samsara.iso
build:
	#cargo bootimage
	cargo build
	mv ./target/x86_64-unknown-none/debug/bootimage-Samsara.bin ./Samsara.bin
clean:
	rm -r ./target
	rm ./Samsara.bin
	rm ./iso/boot/Samsara.bin
