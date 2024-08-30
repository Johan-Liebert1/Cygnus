%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
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
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 16
	
	
	mov QWORD [rbp - 8], 6
	mov QWORD [rbp - 16], string_0
	
	mov rax, 1
	mov rdi, 1
	mov rsi, [rbp - 16]
	mov rdx, [rbp - 8]
	syscall
	
	mov rsp, rbp
	pop rbp
	ret
	

