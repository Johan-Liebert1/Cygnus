%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 104,101,108,108,111,10
	string_1 db 108,111,108,32,104,101,114,101,10
	string_2 db 108,111,108,32,110,111,116,32,104,101,114,101,10
	string_3 db 104,105,10
	string_4 db 98,121,101,10
	string_5 db 52,32,60,32,50,10
	string_6 db 52,32,62,32,50,10
	string_7 db 108,111,111,112,32,119,105,116,104,32,115,116,101,112,32,111,102,32,50,10

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
	sub rsp, 112
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 8], 1
	
	
	
	
	mov QWORD [rbp - 32], 1
	mov QWORD [rbp - 24], 5
	mov QWORD [rbp - 16], 1
	.loop_0:
	
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 8
	mov rax, 5
	cmp rax, rbx
	setle al
	
	.if_0:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_0
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_0
	mov rdx, 6
	syscall
	
	
	
	
	mov QWORD [rbp - 56], 1
	mov QWORD [rbp - 48], 4
	mov QWORD [rbp - 40], 1
	.loop_1:
	
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 3
	mov rax, 3
	cmp rax, rbx
	setne al
	
	.if_1:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_1
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 9
	syscall
	
	jmp .else_end_1
	.if_end_1:
	
	.else_1:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_2
	mov rdx, 13
	syscall
	
	.else_end_1:
	
	.loop_1_end_start:
	;; check exit condition
	mov rcx, [rbp - 56] ;; step
	mov rbx, [rbp - 48] ;; to
	mov rax, [rbp - 40] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_1
	mov [rbp - 40], rax
	jmp .loop_1
	.loop_end_1:
	
	jmp .else_end_0
	.if_end_0:
	
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 2
	mov rax, 6
	cmp rax, rbx
	sete al
	
	.elif_0_0:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_0
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_3
	mov rdx, 3
	syscall
	
	jmp .else_end_0
	.elif_0_0_end:
	
	.else_0:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_4
	mov rdx, 4
	syscall
	
	.else_end_0:
	
	.loop_0_end_start:
	;; check exit condition
	mov rcx, [rbp - 32] ;; step
	mov rbx, [rbp - 24] ;; to
	mov rax, [rbp - 16] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	mov [rbp - 16], rax
	jmp .loop_0
	.loop_end_0:
	
	
	
	
	mov QWORD [rbp - 80], 1
	mov QWORD [rbp - 72], 3
	mov QWORD [rbp - 64], 1
	.loop_2:
	
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 2
	mov rax, 4
	cmp rax, rbx
	setl al
	
	.if_2:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_2
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_5
	mov rdx, 6
	syscall
	
	jmp .else_end_2
	.if_end_2:
	
	.else_2:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_6
	mov rdx, 6
	syscall
	
	.else_end_2:
	
	.loop_2_end_start:
	;; check exit condition
	mov rcx, [rbp - 80] ;; step
	mov rbx, [rbp - 72] ;; to
	mov rax, [rbp - 64] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_2
	mov [rbp - 64], rax
	jmp .loop_2
	.loop_end_2:
	
	
	
	
	mov QWORD [rbp - 104], 2
	mov QWORD [rbp - 96], 11
	mov QWORD [rbp - 88], 1
	.loop_3:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_7
	mov rdx, 20
	syscall
	
	.loop_3_end_start:
	;; check exit condition
	mov rcx, [rbp - 104] ;; step
	mov rbx, [rbp - 96] ;; to
	mov rax, [rbp - 88] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_3
	mov [rbp - 88], rax
	jmp .loop_3
	.loop_end_3:
	
	mov rsp, rbp
	pop rbp
	ret
	

