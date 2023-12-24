%include "std.asm"

section .bss
digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
string_0 db 104,105

section .text
global _start

_start:
mov [argc], rsp
	push 1
	push 30
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
	push 69
	pop rdi
	mov rax, 60
	syscall
	mov rax, string_0
	push rax
	push 2
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	jmp .loop
	.loop_end:
	exit 0

