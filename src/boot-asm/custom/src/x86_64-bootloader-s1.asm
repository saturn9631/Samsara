org 0x7c00
bits 16

boot:
	mov ah, 0x02 ;
	mov al, 0x01 ;al is total sector count. al < 128
	mov ch, 0x00 ;ch is cylinder
	mov cl, 0x02 ;cl sector
	mov dh, 0x00 ;head
	mov dl, 0x00 ;drive number, 00
	mov bx, 0x1000
	mov es, bx
	int 0x13
	jc disk_error
	mov ah, 0x0e
	mov al, "$" ;al is now pin
	mov bh, 0
	int 0x10
	jmp 0x1000:0x00

disk_error:
	mov ah, 0x0e
	mov al, "!"
	mov bh, 0
	int 0x10
	hlt

times 510 - ($ - $$) db 0
dw 0xAA55
