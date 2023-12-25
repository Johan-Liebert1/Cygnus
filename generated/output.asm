%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	one dq 0
	two dq 0
	three dq 0

section .text
	global _start

_start:
	mov [argc], rsp
	push 0
	pop rax
	mov [one], rax
	push 1
	pop rax
	mov [two], rax
	push 0
	pop rax
	mov [three], rax
	mov rax, [one]
	push rax
	pop rax
	call _printRAX
	mov rax, [two]
	push rax
	pop rax
	call _printRAX
	push 1
	push 51
	push 1
	.loop_0:
	pop rcx
	pop rbx
	pop rax
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	inc rax
	inc rbx
	push rax
	push rbx
	push rcx
	mov rax, [one]
	push rax
	mov rax, [two]
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	mov [three], rax
	mov rax, [three]
	push rax
	pop rax
	call _printRAX
	mov rax, [two]
	push rax
	pop rax
	mov [one], rax
	mov rax, [three]
	push rax
	pop rax
	mov [two], rax
	jmp .loop_0
	.loop_end_0:
	exit 0

