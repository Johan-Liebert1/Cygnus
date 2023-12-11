%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8

section .text
	global _start

_start:
	mov rax, 1

	mov rbx, 2

	add rax, rbx
	exit 69

