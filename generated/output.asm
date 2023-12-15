%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8

section .text
	global _start

_start:
mov rax, 13
	mov rbx, 1
	cmp rax, rbx
	jg .skip
	mov rax, 1
	.skip:
	mov rax, 0
	call _printRAX
	exit 0

