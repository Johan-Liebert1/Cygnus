%include "std.asm"

section .bss
digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .text
global _start

_start:
mov [argc], rsp
	push 21
	push 5
	;; get the two operands from the stack
	xor rdx, rdx
	pop rbx
	pop rax
	div rbx
	;; push the remainder result back onto the stack
	push rdx
	pop rax
	call _printRAX
	exit 0

