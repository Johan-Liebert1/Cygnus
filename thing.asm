%include "./generated/std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 97,114,114,97,121,91,53,93,32,61,32

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
	sub rsp, 80
	
	
	
	;; Minus get the two operands from the stack
	mov rbx, 5
	mov rax, 15
	sub rax, rbx
	
	
	mov rbx, rax ;; rbx = 10
	;; rcx stores the index, rdx has the actual value
	mov rcx, 5 ;; rcx = 5
	mov rdx, rbx ;; rdx = 10
	mov rax, 8 ;; rax = 8
	mul rcx ;; rax = 40
	mov rcx, rbp ;; rcx = rbp
	add rcx, rax ;; rcx = rbp + 40
	mov [rcx - 80], rdx ;; rbp - 40 = 10
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_0
	mov rdx, 11
	syscall
	
	
	;; Start array index access
	mov rax, 5
	mov rbx, 8
	mul rbx
	mov rbx, rbp
	add rbx, rax
	mov rax, [rbx - 80]
	
	mov rax, rax
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

