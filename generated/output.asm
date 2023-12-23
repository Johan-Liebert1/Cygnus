%include "std.asm"

section .bss
digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .text
global _start

_start:
mov [argc], rsp
	mov rax, [argc]
	mov rax, [rax + 8]
	push rax
	mov rbx, 0
	.strlen0:
	inc rax
	inc rbx
	mov cl, [rax]
	cmp cl, 0
	jne .strlen0
	push rbx
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	mov rax, [argc]
	mov rax, [rax + 8]
	push rax
	mov rbx, 0
	.strlen1:
	inc rax
	inc rbx
	mov cl, [rax]
	cmp cl, 0
	jne .strlen1
	push rbx
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	mov rax, [argc]
	mov rax, [rax + 8]
	push rax
	mov rbx, 0
	.strlen2:
	inc rax
	inc rbx
	mov cl, [rax]
	cmp cl, 0
	jne .strlen2
	push rbx
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	mov rax, [argc]
	mov rax, [rax + 8]
	push rax
	mov rbx, 0
	.strlen3:
	inc rax
	inc rbx
	mov cl, [rax]
	cmp cl, 0
	jne .strlen3
	push rbx
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	exit 0

