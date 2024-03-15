%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	string_0 db 32,105,115,32,78,79,84,32,97,32,112,97,108,105,110,100,114,111,109,101,10
	string_1 db 32,105,115,32,97,32,112,97,108,105,110,100,114,111,109,101,10
	string_2 db 114,117,115,104
	string_3 db 114,97,99,101,99,97,114

section .text
	global _start

_start:
	mov [argc], rsp
	call _main
	
	exit 0

_is_palindrome:
	push rbp
	mov rbp, rsp
	sub rsp, 64
	;; param name string
	mov [rbp - 8], rax
	;; param name length
	mov [rbp - 16], rdi
	
	push 0
	
	pop rax
	mov [rbp - 24], rax
	
	push 0
	
	mov rax, [rbp - 16]
	push rax
	
	push 2
	
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	push rax
	
	push 1
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 48], rcx
	mov [rbp - 40], rbx
	mov [rbp - 32], rax
	.loop_0:
	mov rcx, [rbp - 48]
	mov rbx, [rbp - 40]
	mov rax, [rbp - 32]
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	inc rax
	inc rbx
	mov [rbp - 40], rbx
	mov [rbp - 32], rax
	
	mov rax, [rbp - 16]
	push rax
	
	mov rax, [rbp - 24]
	push rax
	
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	push rax
	
	push 1
	
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	push rax
	
	pop rax
	mov [rbp - 56], rax
	
	mov rax, [rbp - 8]
	push rax
	
	mov rax, [rbp - 24]
	push rax
	
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; binary op ptr -> char
	mov rbx, rax
	xor rax, rax
	mov al, [rbx]
	push rax
	
	mov rax, [rbp - 8]
	push rax
	
	mov rax, [rbp - 56]
	push rax
	
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; binary op ptr -> char
	mov rbx, rax
	xor rax, rax
	mov al, [rbx]
	push rax
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_0
	mov rax, 0
	jmp .skip_next0
	.skip_0:
	mov rax, 1
	.skip_next0:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	pop rax
	mov [rbp - 64], rax
	
	mov rax, [rbp - 64]
	push rax
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jne .skip_1
	mov rax, 0
	jmp .skip_next1
	.skip_1:
	mov rax, 1
	.skip_next1:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	.if_0:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_0
	
	mov rax, [rbp - 8]
	push rax
	mov rax, [rbp - 16]
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
	
	mov rax, string_0
	push rax
	push 21
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	pop rax
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_0
	.if_end_0:
	
	mov rax, [rbp - 24]
	push rax
	
	push 1
	
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 24], rax
	
	jmp .loop_0
	.loop_end_0:
	
	mov rax, [rbp - 8]
	push rax
	mov rax, [rbp - 16]
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
	
	mov rax, string_1
	push rax
	push 17
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	mov rsp, rbp
	pop rbp
	ret
	

_main:
	push rbp
	mov rbp, rsp
	sub rsp, 32
	
	mov rax, string_2
	push rax
	push 4
	
	pop rbx
	pop rax
	mov [rbp - 16], rbx
	mov [rbp - 8], rax
	
	push 4
	
	mov rax, [rbp - 8]
	push rax
	
	pop rax
	pop rdi
	call _is_palindrome
	
	mov rax, string_3
	push rax
	push 7
	
	pop rbx
	pop rax
	mov [rbp - 32], rbx
	mov [rbp - 24], rax
	
	push 7
	
	mov rax, [rbp - 24]
	push rax
	
	pop rax
	pop rdi
	call _is_palindrome
	
	mov rsp, rbp
	pop rbp
	ret
	

