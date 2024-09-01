%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8
	file resb 4096
	lower_char_occurances resb 208
	upper_char_occurances resb 208

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 46,46,47,101,120,97,109,112,108,101,115,47,116,101,115,116,95,102,105,108,101,115,47,114,101,97,100,95,97,95,102,105,108,101,0

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
	sub rsp, 160
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 8], 97
	
	
	;; Plus get the two operands from the stack
	mov rax, 26
	mov rbx, [rbp - 8]
	add rax, rbx
	;; will lock rax. first = 26. second = [rbp - 8]. Locked: [rax, rbx]
	
	
	;; Minus get the two operands from the stack
	mov rcx, 1
	mov rbx, rax
	sub rbx, rcx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 16], rbx
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], 65
	
	
	;; Plus get the two operands from the stack
	mov rax, 26
	mov rbx, [rbp - 8]
	add rax, rbx
	;; will lock rax. first = 26. second = [rbp - 8]. Locked: [rax, rbx]
	
	
	;; Minus get the two operands from the stack
	mov rcx, 1
	mov rbx, rax
	sub rbx, rcx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], rbx
	
	
	mov QWORD [rbp - 40], 35
	mov QWORD [rbp - 48], string_0
	
	
	mov rax, 2
	
	mov rbx, [rbp - 48]
	
	mov rdi, rbx
	
	
	mov rsi, 0
	
	
	mov rdx, 0
	
	syscall
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 56], rax
	
	mov rax, QWORD [rbp - 56]
	call _printRAX
	
	
	mov rax, 0
	
	mov rdi, [rbp - 56]
	
	xor rbx, rbx
	mov rbx, file
	
	mov rsi, rbx
	
	
	mov rdx, 4096
	
	syscall
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 64], rax
	
	
	mov rax, 1
	
	
	mov rdi, 1
	
	xor rbx, rbx
	mov rbx, file
	
	mov rsi, rbx
	
	mov rdx, [rbp - 64]
	
	syscall
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 64], rax
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 72], 0
	
	
	
	;; loop_0 start
	mov QWORD [rbp - 96], 1 ;; step
	mov rax, [rbp - 64] ;; to
	mov QWORD [rbp - 88], rax ;; to
	mov QWORD [rbp - 80], 0 ;; from
	.loop_0:
	
	xor rax, rax
	mov rax, file
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 72]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 72]. second = rax. Locked: [rax, rbx, rcx]
	
	;; assign_local_pointer of type Integer
	mov rax, rbx
	mov [rbp - 104], rax
	
	mov rbx, [rbp - 104]
	xor rax, rax
	mov al, [rbx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, [rbp - 8]
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovge rax, rbx
	
	mov rcx, [rbp - 104]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, [rbp - 16]
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmovle rbx, rcx
	
	;; gen_logical_statement
	xor rcx, rcx
	mov rcx, rbx
	xor rdx, rdx
	mov rdx, rax
	and rcx, rdx
	
	.if_0:
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_0
	
	mov rbx, [rbp - 104]
	xor rax, rax
	mov al, [rbx]
	
	;; Minus get the two operands from the stack
	mov rcx, [rbp - 8]
	mov rbx, rax
	sub rbx, rcx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 112], rbx
	
	xor rax, rax
	mov rax, lower_char_occurances
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 112]
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = rax. second = rbx. Locked: [rbx, rax, rcx, rdx]
	
	;; assign_local_pointer of type Integer
	mov rax, rcx
	mov [rbp - 120], rax
	
	;; Dereferencing variable addr_to_update
	mov rbx, [rbp - 120]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	
	;; Plus get the two operands from the stack
	mov rbx, 1
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 1. second = rax. Locked: [rax, rbx, rcx]
	
	;; assign_local_pointer of type Integer
	mov rax, rbx
	mov rcx, [rbp - 120]
	mov [rcx], rax
	
	jmp .if_end_0
	.if_end_0:
	
	
	mov rax, [rbp - 72]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 72], rax
	
	.loop_0_end_start:
	;; check exit condition
	mov rcx, [rbp - 96] ;; step
	mov rbx, [rbp - 88] ;; to
	mov rax, [rbp - 80] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	mov [rbp - 80], rax
	jmp .loop_0
	.loop_end_0:
	
	;; assign_local_number of type Integer
	mov rax, [rbp - 8]
	mov QWORD [rbp - 72], rax
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 128], 0
	
	
	
	
	;; loop_1 start
	mov QWORD [rbp - 152], 1 ;; step
	mov QWORD [rbp - 144], 26 ;; to
	mov QWORD [rbp - 136], 0 ;; from
	.loop_1:
	
	mov rax, QWORD [rbp - 72]
	call _printRAX
	
	xor rax, rax
	mov rax, lower_char_occurances
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 128]
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = rax. second = rbx. Locked: [rbx, rax, rcx, rdx]
	mov rcx, [rcx]
	
	mov rax, rcx
	call _printRAX
	
	
	mov rax, [rbp - 128]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 128], rax
	
	
	mov rax, [rbp - 72]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 72], rax
	
	.loop_1_end_start:
	;; check exit condition
	mov rcx, [rbp - 152] ;; step
	mov rbx, [rbp - 144] ;; to
	mov rax, [rbp - 136] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_1
	mov [rbp - 136], rax
	jmp .loop_1
	.loop_end_1:
	
	mov rsp, rbp
	pop rbp
	ret
	

