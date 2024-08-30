%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 46,47,101,120,97,109,112,108,101,115,47,116,101,115,116,95,102,105,108,101,115,47,114,101,97,100,95,97,95,102,105,108,101,0
	string_1 db 102,100,32,61,32

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
	sub rsp, 32
	
	
	mov QWORD [rbp - 8], 34
	mov QWORD [rbp - 16], string_0
	
	
	mov rax, 2
	
	mov rax, [rbp - 16]
	
	mov rdi, rax
	
	
	mov rsi, 0
	
	
	mov rdx, 0
	
	syscall
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 5
	syscall
	
	mov rax, QWORD [rbp - 24]
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

