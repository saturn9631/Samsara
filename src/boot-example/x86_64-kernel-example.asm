org 0
bits 16
section .text
	global _start

_start:
	mov ah, 0x0e
	mov al, "?"
	mov bh, 0x00
	int 0x10
	jmp .done

.done:
	hlt

times 510 - ($ - $$) db 0
dw 0xAA55
