%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	a dq 0
	b dq 0

section .text
	global _start

_start:
	mov [argc], rsp
	push 69
	pop rax
	mov [a], rax
	push 420
	pop rax
	mov [b], rax
	mov rax, [a]
	push rax
	mov rax, [b]
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	exit 0

