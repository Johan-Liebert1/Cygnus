%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8
	current resb 800
	next resb 800

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 32
	string_1 db 42
	string_2 db 10

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
	sub rsp, 192
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 8], 100
	
	xor rax, rax
	mov rax, current
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 8]
	mul rcx
	
	
	
	mov rcx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 3
	mov rsi, 8
	mul rsi
	
	;; Minus get the two operands from the stack
	mov rsi, rax
	mov rdx, rcx
	sub rdx, rsi
	
	;; Plus get the two operands from the stack
	mov rax, rdx
	mov rcx, rbx
	add rax, rcx
	;; will lock rax. first = rdx. second = rbx. Locked: [rbx, rdx, rax, rcx]
	push rax
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 16], rax
	
	xor rax, rax
	mov rax, current
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 8]
	mul rcx
	
	
	
	mov rcx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 2
	mov rsi, 8
	mul rsi
	
	;; Minus get the two operands from the stack
	mov rsi, rax
	mov rdx, rcx
	sub rdx, rsi
	
	;; Plus get the two operands from the stack
	mov rax, rdx
	mov rcx, rbx
	add rax, rcx
	;; will lock rax. first = rdx. second = rbx. Locked: [rbx, rdx, rax, rcx]
	push rax
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 24], rax
	
	xor rax, rax
	mov rax, current
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 8]
	mul rcx
	
	
	;; Minus get the two operands from the stack
	mov rdx, 8
	mov rcx, rax
	sub rcx, rdx
	
	;; Plus get the two operands from the stack
	mov rax, rcx
	mov rdx, rbx
	add rax, rdx
	;; will lock rax. first = rcx. second = rbx. Locked: [rbx, rcx, rax, rdx]
	push rax
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 32], rax
	
	
	;; assign_local_pointer of type Integer
	mov rax, 1
	mov rbx, [rbp - 16]
	mov [rbx], rax
	
	
	;; assign_local_pointer of type Integer
	mov rax, 1
	mov rbx, [rbp - 24]
	mov [rbx], rax
	
	
	;; assign_local_pointer of type Integer
	mov rax, 0
	mov rbx, [rbp - 32]
	mov [rbx], rax
	
	xor rax, rax
	mov rax, current
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 40], rax
	
	xor rax, rax
	mov rax, current
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 48], rax
	
	xor rax, rax
	mov rax, current
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 56], rax
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 64], 0
	
	
	
	;; loop_0 start
	mov QWORD [rbp - 88], 1 ;; step
	mov rax, [rbp - 8] ;; to
	mov QWORD [rbp - 80], rax ;; to
	mov QWORD [rbp - 72], 0 ;; from
	.loop_0:
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 64], 0
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 1
	mov rbx, [rbp - 8]
	add rax, rbx
	;; will lock rax. first = 1. second = [rbp - 8]. Locked: [rax, rbx]
	
	
	;; loop_1 start
	mov QWORD [rbp - 112], 1 ;; step
	mov QWORD [rbp - 104], rax ;; to
	mov QWORD [rbp - 96], 0 ;; from
	.loop_1:
	
	xor rax, rax
	mov rax, current
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 64]
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = rax. second = rbx. Locked: [rbx, rax, rcx, rdx]
	push rax
	
	;; assign_local_pointer of type Integer
	mov rax, rcx
	mov [rbp - 40], rax
	
	xor rax, rax
	mov rax, current
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 64]
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = rax. second = rbx. Locked: [rbx, rax, rcx, rdx]
	push rax
	
	
	;; Plus get the two operands from the stack
	mov rax, 8
	mov rbx, rcx
	add rax, rbx
	;; will lock rax. first = 8. second = rcx. Locked: [rcx, rax, rbx]
	push rax
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 48], rax
	
	xor rax, rax
	mov rax, current
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 64]
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = rax. second = rbx. Locked: [rbx, rax, rcx, rdx]
	push rax
	
	
	;; Plus get the two operands from the stack
	mov rax, 16
	mov rbx, rcx
	add rax, rbx
	;; will lock rax. first = 16. second = rcx. Locked: [rcx, rax, rbx]
	push rax
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 56], rax
	
	xor rax, rax
	mov rax, next
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 64]
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = rax. second = rbx. Locked: [rbx, rax, rcx, rdx]
	push rax
	
	
	;; Plus get the two operands from the stack
	mov rax, 8
	mov rbx, rcx
	add rax, rbx
	;; will lock rax. first = 8. second = rcx. Locked: [rcx, rax, rbx]
	push rax
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 120], rax
	
	;; Dereferencing variable first
	mov rbx, [rbp - 40]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 0
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	;; Dereferencing variable second
	mov rcx, [rbp - 48]
	mov rcx, [rcx]
	xor rbx, rbx
	mov rbx, rcx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, 0
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; Dereferencing variable third
	mov rdx, [rbp - 56]
	mov rdx, [rdx]
	xor rcx, rcx
	mov rcx, rdx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rsi, 0
	mov rdx, rcx
	cmp rdx, rsi
	;; Not xor-ing here as it sets flags
	mov rcx, 0
	mov rdx, 1
	cmove rcx, rdx
	
	;; gen_logical_statement
	xor rdx, rdx
	mov rdx, rcx
	xor rsi, rsi
	mov rsi, rbx
	and rdx, rsi
	
	;; gen_logical_statement
	xor rbx, rbx
	mov rbx, rdx
	xor rcx, rcx
	mov rcx, rax
	and rbx, rcx
	
	.if_0:
	cmp rbx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_0
	
	
	;; assign_local_pointer of type Integer
	mov rax, 0
	mov rbx, [rbp - 120]
	mov [rbx], rax
	
	jmp .elif_0_6_end
	.if_end_0:
	
	;; Dereferencing variable first
	mov rbx, [rbp - 40]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 0
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	;; Dereferencing variable second
	mov rcx, [rbp - 48]
	mov rcx, [rcx]
	xor rbx, rbx
	mov rbx, rcx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, 0
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; Dereferencing variable third
	mov rdx, [rbp - 56]
	mov rdx, [rdx]
	xor rcx, rcx
	mov rcx, rdx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rsi, 1
	mov rdx, rcx
	cmp rdx, rsi
	;; Not xor-ing here as it sets flags
	mov rcx, 0
	mov rdx, 1
	cmove rcx, rdx
	
	;; gen_logical_statement
	xor rdx, rdx
	mov rdx, rcx
	xor rsi, rsi
	mov rsi, rbx
	and rdx, rsi
	
	;; gen_logical_statement
	xor rbx, rbx
	mov rbx, rdx
	xor rcx, rcx
	mov rcx, rax
	and rbx, rcx
	
	.elif_0_0:
	cmp rbx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_0_end
	
	
	;; assign_local_pointer of type Integer
	mov rax, 1
	mov rbx, [rbp - 120]
	mov [rbx], rax
	
	jmp .elif_0_0_end
	.elif_0_0_end:
	
	;; Dereferencing variable first
	mov rbx, [rbp - 40]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 0
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	;; Dereferencing variable second
	mov rcx, [rbp - 48]
	mov rcx, [rcx]
	xor rbx, rbx
	mov rbx, rcx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, 1
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; Dereferencing variable third
	mov rdx, [rbp - 56]
	mov rdx, [rdx]
	xor rcx, rcx
	mov rcx, rdx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rsi, 0
	mov rdx, rcx
	cmp rdx, rsi
	;; Not xor-ing here as it sets flags
	mov rcx, 0
	mov rdx, 1
	cmove rcx, rdx
	
	;; gen_logical_statement
	xor rdx, rdx
	mov rdx, rcx
	xor rsi, rsi
	mov rsi, rbx
	and rdx, rsi
	
	;; gen_logical_statement
	xor rbx, rbx
	mov rbx, rdx
	xor rcx, rcx
	mov rcx, rax
	and rbx, rcx
	
	.elif_0_1:
	cmp rbx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_1_end
	
	
	;; assign_local_pointer of type Integer
	mov rax, 1
	mov rbx, [rbp - 120]
	mov [rbx], rax
	
	jmp .elif_0_1_end
	.elif_0_1_end:
	
	;; Dereferencing variable first
	mov rbx, [rbp - 40]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 0
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	;; Dereferencing variable second
	mov rcx, [rbp - 48]
	mov rcx, [rcx]
	xor rbx, rbx
	mov rbx, rcx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, 1
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; Dereferencing variable third
	mov rdx, [rbp - 56]
	mov rdx, [rdx]
	xor rcx, rcx
	mov rcx, rdx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rsi, 1
	mov rdx, rcx
	cmp rdx, rsi
	;; Not xor-ing here as it sets flags
	mov rcx, 0
	mov rdx, 1
	cmove rcx, rdx
	
	;; gen_logical_statement
	xor rdx, rdx
	mov rdx, rcx
	xor rsi, rsi
	mov rsi, rbx
	and rdx, rsi
	
	;; gen_logical_statement
	xor rbx, rbx
	mov rbx, rdx
	xor rcx, rcx
	mov rcx, rax
	and rbx, rcx
	
	.elif_0_2:
	cmp rbx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_2_end
	
	
	;; assign_local_pointer of type Integer
	mov rax, 1
	mov rbx, [rbp - 120]
	mov [rbx], rax
	
	jmp .elif_0_2_end
	.elif_0_2_end:
	
	;; Dereferencing variable first
	mov rbx, [rbp - 40]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 1
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	;; Dereferencing variable second
	mov rcx, [rbp - 48]
	mov rcx, [rcx]
	xor rbx, rbx
	mov rbx, rcx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, 0
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; Dereferencing variable third
	mov rdx, [rbp - 56]
	mov rdx, [rdx]
	xor rcx, rcx
	mov rcx, rdx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rsi, 0
	mov rdx, rcx
	cmp rdx, rsi
	;; Not xor-ing here as it sets flags
	mov rcx, 0
	mov rdx, 1
	cmove rcx, rdx
	
	;; gen_logical_statement
	xor rdx, rdx
	mov rdx, rcx
	xor rsi, rsi
	mov rsi, rbx
	and rdx, rsi
	
	;; gen_logical_statement
	xor rbx, rbx
	mov rbx, rdx
	xor rcx, rcx
	mov rcx, rax
	and rbx, rcx
	
	.elif_0_3:
	cmp rbx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_3_end
	
	
	;; assign_local_pointer of type Integer
	mov rax, 0
	mov rbx, [rbp - 120]
	mov [rbx], rax
	
	jmp .elif_0_3_end
	.elif_0_3_end:
	
	;; Dereferencing variable first
	mov rbx, [rbp - 40]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 1
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	;; Dereferencing variable second
	mov rcx, [rbp - 48]
	mov rcx, [rcx]
	xor rbx, rbx
	mov rbx, rcx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, 0
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; Dereferencing variable third
	mov rdx, [rbp - 56]
	mov rdx, [rdx]
	xor rcx, rcx
	mov rcx, rdx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rsi, 1
	mov rdx, rcx
	cmp rdx, rsi
	;; Not xor-ing here as it sets flags
	mov rcx, 0
	mov rdx, 1
	cmove rcx, rdx
	
	;; gen_logical_statement
	xor rdx, rdx
	mov rdx, rcx
	xor rsi, rsi
	mov rsi, rbx
	and rdx, rsi
	
	;; gen_logical_statement
	xor rbx, rbx
	mov rbx, rdx
	xor rcx, rcx
	mov rcx, rax
	and rbx, rcx
	
	.elif_0_4:
	cmp rbx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_4_end
	
	
	;; assign_local_pointer of type Integer
	mov rax, 1
	mov rbx, [rbp - 120]
	mov [rbx], rax
	
	jmp .elif_0_4_end
	.elif_0_4_end:
	
	;; Dereferencing variable first
	mov rbx, [rbp - 40]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 1
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	;; Dereferencing variable second
	mov rcx, [rbp - 48]
	mov rcx, [rcx]
	xor rbx, rbx
	mov rbx, rcx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, 1
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; Dereferencing variable third
	mov rdx, [rbp - 56]
	mov rdx, [rdx]
	xor rcx, rcx
	mov rcx, rdx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rsi, 0
	mov rdx, rcx
	cmp rdx, rsi
	;; Not xor-ing here as it sets flags
	mov rcx, 0
	mov rdx, 1
	cmove rcx, rdx
	
	;; gen_logical_statement
	xor rdx, rdx
	mov rdx, rcx
	xor rsi, rsi
	mov rsi, rbx
	and rdx, rsi
	
	;; gen_logical_statement
	xor rbx, rbx
	mov rbx, rdx
	xor rcx, rcx
	mov rcx, rax
	and rbx, rcx
	
	.elif_0_5:
	cmp rbx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_5_end
	
	
	;; assign_local_pointer of type Integer
	mov rax, 1
	mov rbx, [rbp - 120]
	mov [rbx], rax
	
	jmp .elif_0_5_end
	.elif_0_5_end:
	
	;; Dereferencing variable first
	mov rbx, [rbp - 40]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 1
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	;; Dereferencing variable second
	mov rcx, [rbp - 48]
	mov rcx, [rcx]
	xor rbx, rbx
	mov rbx, rcx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, 1
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; Dereferencing variable third
	mov rdx, [rbp - 56]
	mov rdx, [rdx]
	xor rcx, rcx
	mov rcx, rdx
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rsi, 1
	mov rdx, rcx
	cmp rdx, rsi
	;; Not xor-ing here as it sets flags
	mov rcx, 0
	mov rdx, 1
	cmove rcx, rdx
	
	;; gen_logical_statement
	xor rdx, rdx
	mov rdx, rcx
	xor rsi, rsi
	mov rsi, rbx
	and rdx, rsi
	
	;; gen_logical_statement
	xor rbx, rbx
	mov rbx, rdx
	xor rcx, rcx
	mov rcx, rax
	and rbx, rcx
	
	.elif_0_6:
	cmp rbx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_6_end
	
	
	;; assign_local_pointer of type Integer
	mov rax, 0
	mov rbx, [rbp - 120]
	mov [rbx], rax
	
	jmp .elif_0_6_end
	.elif_0_6_end:
	
	
	;; Plus get the two operands from the stack
	mov rax, 1
	mov rbx, [rbp - 64]
	add rax, rbx
	;; will lock rax. first = 1. second = [rbp - 64]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 64], rax
	
	.loop_1_end_start:
	;; check exit condition
	mov rcx, [rbp - 112] ;; step
	mov rbx, [rbp - 104] ;; to
	mov rax, [rbp - 96] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_1
	mov [rbp - 96], rax
	jmp .loop_1
	.loop_end_1:
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 128], 0
	
	
	
	;; loop_2 start
	mov QWORD [rbp - 152], 1 ;; step
	mov rax, [rbp - 8] ;; to
	mov QWORD [rbp - 144], rax ;; to
	mov QWORD [rbp - 136], 0 ;; from
	.loop_2:
	
	xor rax, rax
	mov rax, next
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 128]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 128]. second = rax. Locked: [rax, rbx, rcx]
	mov rax, [rax]
	push rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 0
	mov rax, rbx
	cmp rax, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_1:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_1
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_0
	mov rdx, 1
	syscall
	
	jmp .else_end_1
	.if_end_1:
	
	.else_1:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 1
	syscall
	
	.else_end_1:
	
	
	;; Plus get the two operands from the stack
	mov rax, 8
	mov rbx, [rbp - 128]
	add rax, rbx
	;; will lock rax. first = 8. second = [rbp - 128]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 128], rax
	
	.loop_2_end_start:
	;; check exit condition
	mov rcx, [rbp - 152] ;; step
	mov rbx, [rbp - 144] ;; to
	mov rax, [rbp - 136] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_2
	mov [rbp - 136], rax
	jmp .loop_2
	.loop_end_2:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_2
	mov rdx, 1
	syscall
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 128], 0
	
	
	
	;; loop_3 start
	mov QWORD [rbp - 176], 1 ;; step
	mov rax, [rbp - 8] ;; to
	mov QWORD [rbp - 168], rax ;; to
	mov QWORD [rbp - 160], 0 ;; from
	.loop_3:
	
	xor rax, rax
	mov rax, current
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 128]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 128]. second = rax. Locked: [rax, rbx, rcx]
	push rax
	
	;; assign_local_pointer of type Integer
	mov rax, rbx
	mov [rbp - 184], rax
	
	xor rax, rax
	mov rax, next
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 128]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 128]. second = rax. Locked: [rax, rbx, rcx]
	push rax
	
	;; assign_local_pointer of type Integer
	mov rax, rbx
	mov [rbp - 192], rax
	
	;; Dereferencing variable idx_into_next
	mov rbx, [rbp - 192]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov rcx, [rbp - 184]
	mov [rcx], rbx
	
	
	;; Plus get the two operands from the stack
	mov rax, 8
	mov rbx, [rbp - 128]
	add rax, rbx
	;; will lock rax. first = 8. second = [rbp - 128]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 128], rax
	
	.loop_3_end_start:
	;; check exit condition
	mov rcx, [rbp - 176] ;; step
	mov rbx, [rbp - 168] ;; to
	mov rax, [rbp - 160] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_3
	mov [rbp - 160], rax
	jmp .loop_3
	.loop_end_3:
	
	.loop_0_end_start:
	;; check exit condition
	mov rcx, [rbp - 88] ;; step
	mov rbx, [rbp - 80] ;; to
	mov rax, [rbp - 72] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	mov [rbp - 72], rax
	jmp .loop_0
	.loop_end_0:
	
	mov rsp, rbp
	pop rbp
	ret
	

