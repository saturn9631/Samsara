; 0x7C00 -> 0xAA55
org 0x7c00
bits 16

JMP SHORT main
NOP

bdb_oem: DB 'MSWIN4.1'
bdb_bytes_per_sector: DW 512
bdb_sectors_per_cluster: DB 1
bdb_reserved_sectors: DW 1
bdb_fat_count: DB 2
bdb_dir_entries_count: DW 0E0h
bdb_total_sectors: DW 2880
bdb_media_descriptor_type: DB 0F0h
bdb_sectors_per_fat: DW 9
bdb_sectors_per_track: DW 2
bdb_heads: DW 2
bdb_hidden_sectors: DD 0
bdb_large_sector_count: DD 0

ebr_drive_number: DB 0
		  DB 0
ebr_signature:	  DB	 29h
evbr_volume_id:   DB 12h,34h,56h,78h
ebr_volume_labal: DB 'SAMASARA OS' ;has to be 11 characters long
ebr_system_id: DB 'FAT12   ' ;has to 8 characters long

main:
	MOV ax, 0
	MOV ds, ax
	MOV es, ax
	MOV ss, ax
	MOV sp, 0x7c00

	MOV dl, [ebr_drive_number]
	MOV ax, 1
	MOV cl, 1
	MOV bx, 0x7E00
	MOV si, os_boot_msg
	CALL print


	;FAT is made of 4 segments
	;reserved segment: 1 segment
	;FAT: bdb_sectors_per_fat * bdb_fat_count or 9 * 2 = 18
	;root directory:
	;data

	MOV ax, [bdb_sectors_per_fat]
	MOV bl, [bdb_fat_count] 
	XOR bh, bh
	MUL bx
	ADD ax, [bdb_reserved_sectors] ;LBA of the root directory
	PUSH ax

	MOV ax, [bdb_dir_entries_count]
	SHL ax, 5 ;ax *= 32
	XOR dx, dx
	DIV word [bdb_bytes_per_sector] ;(32 * num of entries) / bytes per sector

	TEST dx, dx
	JZ root_dir_after
	INC ax
	HLT
	JMP halt

root_dir_after:
	MOV cl, al
	POP ax
	MOV dl, [ebr_drive_number]
	MOV bx, buffer
	CALL disk_read
	XOR bx, bx
	MOV di, buffer

search_kernel:
	MOV si, file_kernel_bin
	MOV cx, 11
	PUSH di
	REPE CMPSB

	POP di
	JE found_kernel

	ADD di, 32
	INC bx
	CMP bx, [bdb_dir_entries_count]
	JL search_kernel
	JMP kernel_not_found

kernel_not_found:
	MOV si, msg_kernel_not_found
	CALL print

	JMP halt

found_kernel:
	MOV ax, [di+26]
	MOV [kernel_cluster], ax

	MOV ax, [bdb_reserved_sectors]
	MOV bx, buffer
	MOV cl, [bdb_sectors_per_fat]
	MOV dl, [ebr_drive_number]

	CALL disk_read

	MOV bx, kernel_load_segment
	MOV es, bx
	MOV bx, kernel_load_offset

load_kernel_loop:
	MOV ax, [kernel_cluster]
	ADD ax, 31 ;hardcoded value, change when using a file system other than FAT
	MOV cl, 1
	MOV dl, [ebr_drive_number]
	CALL disk_read
	ADD bx, [bdb_bytes_per_sector]
	MOV ax, [kernel_cluster] ;(kernel_cluster * 3) / 2
	MOV cx, 3
	MUL cx
	MOV cx, 2
	DIV cx

	MOV si, buffer
	ADD si, ax
	MOV ax, [ds:si]

	OR dx, dx
	jz even

odd:
	SHR ax, 4
	JMP next_cluster_after
even:
	AND ax, 4
	JMP next_cluster_after

next_cluster_after:
	CMP ax, 0x0FF8
	JAE read_finish

	MOV [kernel_cluster], ax
	JMP load_kernel_loop

read_finish:
	MOV dl, [ebr_drive_number]
	MOV ax, kernel_load_segment
	MOV ds, ax
	MOV es, ax

	JMP kernel_load_segment:kernel_load_offset

	HLT
	JMP halt


halt:
	HLT
	JMP halt

disk_read:
	PUSH ax
	PUSH bx
	PUSH cx
	PUSH dx
	PUSH di

	call lba_to_chs

	MOV ah, 02h
	MOV di, 3 ;counter

retry:
	STC
	INT 13h
	JNC done_read
	CALL diskReset
	DEC di
	TEST di, di
	JNZ retry

failDiskRead:
	MOV si, read_failure
	CALL print
	HLT
	JMP halt

diskReset:
	PUSHA
	MOV ah, 0
	STC
	INT 13h
	JC failDiskRead
	POPA

done_read:
	pop di
	pop dx
	pop cx
	pop bx
	pop ax

; input: LBA index in ax
; cx [bits 0-5]: sector number
; cx [bits 6-15]: cylinder
; dh: head
lba_to_chs:
	PUSH ax
	PUSH dx
	
	XOR dx,dx
	DIV word [bdb_sectors_per_track] ;LBA % sectors per track) + 1 <- sector
	INC dx ;Sector
	MOV cx, dx
	XOR dx,dx
	DIV word [bdb_heads]
	MOV dh, dl ;head
	MOV ch, al
	SHL ah, 6
	OR cl, ah ; cylinder
	POP ax
	MOV dl, al
	POP ax ;Might be POP dx

	;head: (LBA/secotors per track) % number of heads
	;cylinder: (LBA / sectors per track) / number of heads
	RET

print:
	PUSH si
	PUSH ax
	PUSH bx

print_loop:
	LODSB
	OR al, al ;checks to see if al is zero, in which case the end of the string has been reached
	JZ done_print ; jumps if al is zero
	MOV ah, 0x0e ;tells interrupt 0x10 to print character
	MOV bh, 0 ; page number of the 0x10 interrupt. Changing the page number to a none-zero value causes a different display than standard to be used.
	INT 0x10 ;video interrupt
	JMP print_loop

done_print:
	POP ax
	POP bx
	POP si
	RET

os_boot_msg: DB 'Our Bootloader has started!', 0x0d, 0x0a, 0
read_failure: DB 'Failed to read disk', 0x0d, 0x0a, 0
file_kernel_bin: DB 'KERNEL  BIN'
msg_kernel_not_found: DB 'KERNEL.BIN not found!'
kernel_cluster: DW 0 
kernel_load_segment EQU 0x2000
kernel_load_offset EQU 0

TIMES 510 - ($ - $$) DB 0
DW 0AA55h

buffer:
