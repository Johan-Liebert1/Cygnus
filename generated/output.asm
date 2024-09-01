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

_add:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 16
	;; param name a
	mov [rbp - 8], rdi
	;; param name b
	mov [rbp - 16], rsi
	
	;; Plus get the two operands from the stack
	mov rax, [rbp - 16]
	mov rbx, [rbp - 8]
	add rax, rbx
	;; will lock rax. first = [rbp - 16]. second = [rbp - 8]. Locked: [rax, rbx]
	
	mov rax, rax
	
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
	sub rsp, 16
	
	
	;; Moving argument number 1
	mov rdi, 1
	
	push rdi
	
	
	;; Moving argument number 1
	mov rdi, 2
	
	
	;; Moving argument number 2
	mov rsi, 3
	
	pop rdi
	
	call _add
	
	;; Moving argument number 2
	mov rsi, rax
	
	call _add
	
	mov rax, rax
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

