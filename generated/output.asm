%include "std.asm"

section .bss
digitSpace resb 100
	digitSpacePos resb 8

section .text
global _start

_start:
push 1024
	push 9
	;; get the two operands from the stack
	pop rcx
	pop rax
	;; We can only shift left or right by 8 bits
	shr rax, cl
	;; push the result back onto the stack
	push rax
	push 2
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	exit 0

