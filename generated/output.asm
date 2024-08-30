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
	sub rsp, 32
	
	
	
	
	mov QWORD [rbp - 24], 1
	mov QWORD [rbp - 16], 5
	mov QWORD [rbp - 8], 1
	mov QWORD [rbp - 32], 1
	.loop_0:
	
	mov rax, QWORD [rbp - 32]
	call _printRAX
	
	.loop_0_end_start:
	;; inc the loop variable
	mov rdx, [rbp - 32]
	mov rcx, [rbp - 24]
	add rdx, rcx
	mov [rbp - 32], rdx
	;; check exit condition
	mov rcx, [rbp - 24] ;; step
	mov rbx, [rbp - 16] ;; to
	mov rax, [rbp - 8] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	mov [rbp - 8], rax
	jmp .loop_0
	.loop_end_0:
	
	mov rsp, rbp
	pop rbp
	ret
	

