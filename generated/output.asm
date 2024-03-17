%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	string_0 db 104,101,108,108,111,10

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
	push 6
	
	push 69
	
	push 2
	
	pop rax
	mov [rbp - 16], rax
	
	pop rax
	mov [rbp - 8], rax
	
	pop rbx
	pop rax
	mov [rbp - 32], rbx
	mov [rbp - 24], rax
	
	mov rax, [rbp - 24]
	push rax
	mov rax, [rbp - 32]
	push rax
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	mov rax, [rbp - 16]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, [rbp - 8]
	push rax
	
	pop rax
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

