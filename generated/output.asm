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
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 8], 5
	
	mov rax, QWORD [rbp - 8]
	call _printRAX
	
	
	mov rax, 10
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

