%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 114,97,99,101,99,97,114

section .text
	global _start

_start:
	mov [argc], rsp
	call _main
	
	exit 0

_is_palindrome:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 64
	;; param name length
	mov [rbp - 8], rdi
	;; param name string
	mov [rbp - 16], rsi
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], 0
	
	
	
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rbx, 2
	mov rax, [rbp - 8]
	div rbx
	
	
	;; Plus get the two operands from the stack
	mov rbx, 1
	mov rcx, rax
	add rbx, rcx
	
	
	;; loop_0 start
	mov QWORD [rbp - 48], 1 ;; step
	mov QWORD [rbp - 40], rbx ;; to
	mov QWORD [rbp - 32], 0 ;; from
	.loop_0:
	
	;; Minus get the two operands from the stack
	mov rbx, [rbp - 24]
	mov rax, [rbp - 8]
	sub rax, rbx
	
	
	;; Minus get the two operands from the stack
	mov rcx, 1
	mov rbx, rax
	sub rbx, rcx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 56], rbx
	
	mov rax, [rbp - 16]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 24]
	mov rcx, rax
	add rbx, rcx
	;; binary op ptr -> char
	mov rbx, rax
	xor rax, rax
	mov al, [rbx]
	push rax
	
	mov rax, [rbp - 16]
	
	;; Plus get the two operands from the stack
	mov rcx, [rbp - 56]
	mov rdx, rax
	add rcx, rdx
	;; binary op ptr -> char
	mov rbx, rax
	xor rax, rax
	mov al, [rbx]
	push rax
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, rcx
	mov rax, rbx
	cmp rax, rbx
	xor rax, rax
	sete al
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 64], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 1
	mov rax, [rbp - 64]
	cmp rax, rbx
	xor rax, rax
	setne al
	
	.if_0:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_0
	
	
	mov rax, 0
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_0
	.if_end_0:
	
	
	;; Plus get the two operands from the stack
	mov rax, 1
	mov rbx, [rbp - 24]
	add rax, rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], rax
	
	.loop_0_end_start:
	;; check exit condition
	mov rcx, [rbp - 48] ;; step
	mov rbx, [rbp - 40] ;; to
	mov rax, [rbp - 32] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	mov [rbp - 32], rax
	jmp .loop_0
	.loop_end_0:
	
	
	mov rax, 1
	
	mov rsp, rbp
	pop rbp
	ret
	
	mov rsp, rbp
	pop rbp
	ret
	

_main:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 32
	
	
	mov QWORD [rbp - 8], 7
	mov QWORD [rbp - 16], string_0
	
	
	;; Moving argument number 1
	mov rdi, 2
	
	mov rax, [rbp - 16]
	
	;; Moving argument number 2
	mov rsi, rax
	
	call _is_palindrome
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], rax
	
	mov rax, QWORD [rbp - 24]
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

