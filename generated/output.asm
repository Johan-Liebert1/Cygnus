%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0

section .text
	global _start

_start:
	mov [argc], rsp
	call _main
	
	exit 0

_main:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 16
	
	push 30
	
	push 20
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 16], rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 8], rax
	
	xor rax, rax
	mov rax, [rbp - 16]
	push rax
	
	pop rax
	call _printRAX
	
	xor rax, rax
	mov rax, [rbp - 8]
	push rax
	
	pop rax
	call _printRAX
	
	xor rax, rax
	mov rax, [rbp - 16]
	push rax
	
	push 15
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 16], rax
	
	xor rax, rax
	mov rax, [rbp - 8]
	push rax
	
	push 15
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 8], rax
	
	xor rax, rax
	mov rax, [rbp - 16]
	push rax
	
	pop rax
	call _printRAX
	
	xor rax, rax
	mov rax, [rbp - 8]
	push rax
	
	pop rax
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

