%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8

section .text
	global _start

_start:
push 1
	push 10
	push 1
	.loop:
	pop rcx
	pop rbx
	pop rax
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end
	inc rax
	inc rbx
	push rax
	push rbx
	push rcx
	push 1
	pop rax
	call _printRAX
	jmp .loop
	.loop_end:
	exit 0

