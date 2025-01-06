%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 104,101,108,108,111

section .text
	global _start

_start:
	mov [argc], rsp
	call _main
	
	exit 0

_get_len:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 16
	
	
	mov rax, 5
	
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
	
	
	mov QWORD [rbp - 8], 5
	mov QWORD [rbp - 16], string_0
	
	
	mov rax, 1
	
	
	mov rdi, 1
	
	mov rsi, [rbp - 16]
	
	
	mov rdx, 5
	
	syscall
	
	
	mov rsp, rbp
	pop rbp
	ret
	

