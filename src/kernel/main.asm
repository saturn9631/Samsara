; 0x7C00 -> 0xAA55
org 0x7c00
bits 16

main:
	mov ax, 0
	mov ds, ax
	mov es, ax
	mov ss, ax
	mov sp, 0x7c00
	mov si, os_boot_msg
	call print
	hlt

halt:
	jmp halt

print:
	push si
	push ax
	push bx

print_loop:
	lodsb
	or al, al ;checks to see if al is zero, in which case the end of the string has been reached
	jz done_print ; jumps if al is zero
	mov ah, 0x0e ;tells interrupt 0x10 to print character
	mov bh, 0 ; page number of the 0x10 interrupt. Changing the page number to a none-zero value causes a different display than standard to be used.
	int 0x10 ;video interrupt
	jmp print_loop

done_print:
	pop ax
	pop bx
	pop si
	ret

os_boot_msg: db 'Our OS has booted!', 0x0d, 0x0a, 0

times 510-($-$$) db 0
dw 0AA55h
