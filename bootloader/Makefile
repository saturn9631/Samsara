build:
	cargo rustc --target x86_64-unknown-uefi 
	cp target/x86_64-unknown-uefi/debug/bootloader.efi ./bootloader.efi
	cp ./bootloader.efi ./esp/efi/boot/bootx64.efi
boot: build
	qemu-system-x86_64 -enable-kvm \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
		-drive format=raw,file=fat:rw:esp
clean:
	rm -r target/
	rm bootloader.efi
	rm ./esp/efi/boot/bootx64.efi

