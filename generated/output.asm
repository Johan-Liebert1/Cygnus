%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	a db 0

section .text
	global _start

_start:
	mov [argc], rsp
	push 21
	pop rax
	mov [a], rax
	push 32
	push 45
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	push 76
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	mov [a], rax

	exit 0

