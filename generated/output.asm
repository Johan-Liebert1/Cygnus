%include "std.asm"

section .bss
digitSpace resb 100
	digitSpacePos resb 8

section .text
global _start

_start:
push 10
	push 5
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	exit 0

