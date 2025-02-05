
section .multiboot_header
header_start:
	DD 0xe85250d6 ;multiboot 2 magic number
	DD 0 ;protected mode i386
	DD header_end - header_start ;length
	DD 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
	DW 0 
	DW 0
	DD 8
header_end:
