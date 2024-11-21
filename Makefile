
#
# Variables
#
ASM=nasm
SRC_DIR=src
BUILD_DIR=build
EDITOR=nvim
VM=qemu-system-i386 -fda
VM2= qemu-system-i386 -boot c -m 256 -hda
VM2SUFFIX= -s -S

#
# Utils
#
edit:
	$(EDITOR) $(SRC_DIR)/bootloader/boot.asm $(SRC_DIR)/kernel/main.asm
edit-bin:
	hexedit build/main.img

clean:
	rm -r $(BUILD_DIR)
	mkdir $(BUILD_DIR)

boot: floppy_image
	$(VM) build/main.img

debug: floppy_image
	$(VM2) build/main.img $(VM2SUFFIX) &

#
# Floppy Image
#
floppy_image: $(BUILD_DIR)/main.img

$(BUILD_DIR)/main.img: bootloader kernel
	dd if=/dev/zero of=$(BUILD_DIR)/main.img bs=512 count=2880
	mkfs.fat -F 12 -n "SAMSARA" $(BUILD_DIR)/main.img
	dd if=$(BUILD_DIR)/bootloader.bin of=$(BUILD_DIR)/main.img conv=notrunc
	mcopy -i $(BUILD_DIR)/main.img $(BUILD_DIR)/kernel.bin "::kernel.bin"


#
# Bootloader
#
bootloader: $(BUILD_DIR)/bootloader.bin

$(BUILD_DIR)/bootloader.bin:
	$(ASM) $(SRC_DIR)/bootloader/boot.asm -f bin -o $(BUILD_DIR)/bootloader.bin


#
# Kernel
#
kernel: $(BUILD_DIR)/kernel.bin

$(BUILD_DIR)/kernel.bin:
	$(ASM) $(SRC_DIR)/kernel/main.asm -f bin -o $(BUILD_DIR)/kernel.bin

