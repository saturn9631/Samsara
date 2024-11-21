; 0x7C00 -> 0xAA55
org 0x7c00
bits 16

JMP SHORT main
NOP

bdb_oem: DB 'MSWIN4.1'
bdb_bytes_per_sector: DW 512
bdb_sectors_per_cluster: DB 1
bdb_reseverd_sectors: DW 1
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
	CALL disk_read

	HLT

halt:
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

os_boot_msg: DB 'Our OS has booted!', 0x0d, 0x0a, 0
read_failure: DB 'Failed to read disk', 0x0d, 0x0a, 0

TIMES 510 - ($ - $$) DB 0
DW 0AA55h
