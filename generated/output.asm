%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 97,114,114,97,121,91,105,93,32,61,32

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
	sub rsp, 464
	
	
	
	
	;; loop_0 start
	mov QWORD [rbp - 424], 1
	mov QWORD [rbp - 416], 50
	mov QWORD [rbp - 408], 0
	mov QWORD [rbp - 432], 0
	.loop_0:
	
	
	;; Minus get the two operands from the stack
	mov rbx, [rbp - 432]
	mov rax, 50
	sub rax, rbx
	
	;; rbx stores the index, rcx has the actual value
	mov rcx, [rbp - 432]
	mov rdx, rax
	mov rbx, 8
	mul rcx
	mov rcx, rbp
	add rcx, rbx
	mov [rcx - 400], rdx
	
	.loop_0_end_start:
	;; inc the loop variable
	mov rax, [rbp - 432]
	mov rbx, [rbp - 424]
	add rax, rbx
	mov [rbp - 432], rax
	;; check exit condition
	mov rcx, [rbp - 424] ;; step
	mov rbx, [rbp - 416] ;; to
	mov rax, [rbp - 408] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	mov [rbp - 408], rax
	jmp .loop_0
	.loop_end_0:
	
	
	
	
	;; loop_1 start
	mov QWORD [rbp - 456], 1
	mov QWORD [rbp - 448], 50
	mov QWORD [rbp - 440], 0
	mov QWORD [rbp - 464], 0
	.loop_1:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_0
	mov rdx, 11
	syscall
	
	;; Start array index access
	mov rax, [rbp - 464]
	mov rbx, 8
	mul rbx
	mov rbx, rbp
	add rbx, rax
	mov rax, [rbx - 400]
	
	mov rax, rax
	call _printRAX
	
	.loop_1_end_start:
	;; inc the loop variable
	mov rbx, [rbp - 464]
	mov rcx, [rbp - 456]
	add rbx, rcx
	mov [rbp - 464], rbx
	;; check exit condition
	mov rdx, [rbp - 456] ;; step
	mov rcx, [rbp - 448] ;; to
	mov rbx, [rbp - 440] ;; from
	add rbx, rdx
	dec rcx
	cmp rbx, rcx
	jg .loop_end_1
	mov [rbp - 440], rbx
	jmp .loop_1
	.loop_end_1:
	
	mov rsp, rbp
	pop rbp
	ret
	

