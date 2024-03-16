%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	string_0 db 104,101,108,108,111

section .text
	global _start

_start:
	mov [argc], rsp
	call _main
	
	exit 0

_main:
	push rbp
	mov rbp, rsp
	sub rsp, 32
	
	mov rax, string_0
	push rax
	push 5
	
	push 1
	
	push 2
	
	pop rax
	mov [rbp - 16], rax
	
	pop rax
	mov [rbp - 8], rax
	
	pop rbx
	pop rax
	mov [rbp - 32], rbx
	mov [rbp - 24], rax
	
	mov rsp, rbp
	pop rbp
	ret
	

