%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	float_0 dq 3.14
	float_1 dq 6.28
	float_2 dq 3.14
	float_3 dq 6.28
	float_4 dq 3.14
	float_5 dq 6.28
	float_6 dq 3.14
	float_7 dq 6.28
	float_8 dq 3.14
	float_9 dq 6.28
	float_10 dq 3.14

section .text
	global _start

_start:
	mov [argc], rsp
	call _main
	
	exit 0

_add:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 16
	;; param name a
	mov [rbp - 8], rdi
	;; param name b
	mov [rbp - 16], rsi
	
	movsd xmm0, [float_0]
	
	;; assign_local_pointer of type Floating Point
	movsd xmm1, xmm0
	mov rax, [rbp - 8]
	movsd [rax], xmm1
	
	;; Dereferencing variable a. handle_local_ptr_int_float
	mov rax, [rbp - 8]
	mov rax, [rax]
	mov [float_imm], rax
	movsd xmm0, [float_imm]
	
	;; Dereferencing variable b. handle_local_ptr_int_float
	mov rax, [rbp - 16]
	mov rax, [rax]
	mov [float_imm], rax
	movsd xmm1, [float_imm]
	
	;; Plus get the two operands from the stack
	movsd xmm2, xmm1
	movsd xmm3, xmm0
	addsd xmm2, xmm3
	
	;; write float
	movsd [float_imm], xmm2
	mov rax, [float_imm]
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

_main:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 16
	
	movsd xmm0, [float_1]
	
	movsd [rbp - 8], xmm0
	
	movsd xmm0, [float_2]
	
	movsd [rbp - 16], xmm0
	
	movsd xmm0, [float_3]
	
	movsd xmm1, [float_4]
	
	;; Plus get the two operands from the stack
	movsd xmm2, xmm1
	movsd xmm3, xmm0
	addsd xmm2, xmm3
	
	;; write float
	movsd [float_imm], xmm2
	mov rax, [float_imm]
	call _printRAX
	
	movsd xmm0, [float_5]
	
	movsd xmm1, [float_6]
	
	;; Plus get the two operands from the stack
	movsd xmm2, xmm0
	movsd xmm3, xmm1
	subsd xmm2, xmm3
	
	;; write float
	movsd [float_imm], xmm2
	mov rax, [float_imm]
	call _printRAX
	
	movsd xmm0, [float_7]
	
	movsd xmm1, [float_8]
	
	;; Plus get the two operands from the stack
	movsd xmm2, xmm1
	movsd xmm3, xmm0
	mulsd xmm2, xmm3
	
	;; write float
	movsd [float_imm], xmm2
	mov rax, [float_imm]
	call _printRAX
	
	movsd xmm0, [float_9]
	
	movsd xmm1, [float_10]
	
	;; Plus get the two operands from the stack
	movsd xmm2, xmm0
	movsd xmm3, xmm1
	divsd xmm2, xmm3
	
	;; write float
	movsd [float_imm], xmm2
	mov rax, [float_imm]
	call _printRAX
	
	lea rax, [rbp - 8]
	
	;; Moving argument number 1
	mov rdi, rax
	
	lea rax, [rbp - 16]
	
	;; Moving argument number 2
	mov rsi, rax
	
	call _add
	
	;; Writing float variable
	mov rax, [rbp - 8]
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

