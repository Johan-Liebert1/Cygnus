%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 102,105,114,115,116,32,102,114,111,109,32,61,32
	string_1 db 102,105,114,115,116,32,115,116,101,112,32,61,32
	string_2 db 70,105,114,115,116,32,108,111,111,112,32,105,32,61,32
	string_3 db 10
	string_4 db 83,101,99,111,110,100,32,108,111,111,112,32,105,32,61,32
	string_5 db 111,117,116,101,114,32,105,32,61,32

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
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 8], 4
	
	
	
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 32], rcx
	mov [rbp - 24], rbx
	mov [rbp - 16], rax
	mov [rbp - 40], rax
	.loop_0:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_0
	mov rdx, 13
	syscall
	
	mov rax, QWORD [rbp - 16]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 13
	syscall
	
	mov rax, QWORD [rbp - 32]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_2
	mov rdx, 15
	syscall
	
	mov rax, QWORD [rbp - 40]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_3
	mov rdx, 1
	syscall
	
	.loop_0_end_start:
	;; inc the loop variable
	mov rdx, [rbp - 40]
	mov rcx, [rbp - 32]
	add rdx, rcx
	mov [rbp - 40], rdx
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
	
	
	
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 64], rcx
	mov [rbp - 56], rbx
	mov [rbp - 48], rax
	mov [rbp - 72], rax
	.loop_1:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_4
	mov rdx, 16
	syscall
	
	mov rax, QWORD [rbp - 72]
	call _printRAX
	
	.loop_1_end_start:
	;; inc the loop variable
	mov rdx, [rbp - 72]
	mov rcx, [rbp - 64]
	add rdx, rcx
	mov [rbp - 72], rdx
	;; check exit condition
	mov rcx, [rbp - 64] ;; step
	mov rbx, [rbp - 56] ;; to
	mov rax, [rbp - 48] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_1
	mov [rbp - 48], rax
	jmp .loop_1
	.loop_end_1:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_5
	mov rdx, 10
	syscall
	
	mov rax, QWORD [rbp - 8]
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

