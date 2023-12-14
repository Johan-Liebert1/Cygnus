%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8

section .text
	global _start

_start:
	mov rax, 400

	mov rbx, 900

	add rax, rbx

	 call _printRAX
	exit 0

