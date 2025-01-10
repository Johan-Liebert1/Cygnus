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
	string_0 db 46,47,101,120,97,109,112,108,101,115,47,116,101,115,116,95,102,105,108,101,115,47,114,101,97,100,95,97,95,102,105,108,101,0
	string_1 db 114,101,97,100,32,102,100,32,61,32

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
	sub rsp, 144
	
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 1], 97
	
	
	;; Plus get the two operands from the stack
	mov rax, 26
	mov rbx, [rbp - 1]
	add rax, rbx
	;; will lock rax. first = 26. second = [rbp - 1]. Locked: [rax, rbx]
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 271 ./examples/count_char_occurances_in_file.cy:13:32 Op(Minus) >
	mov rcx, 1
	mov rbx, rax
	sub rbx, rcx
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 2], bl
	
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 3], 65
	
	
	;; Plus get the two operands from the stack
	mov rax, 26
	mov rbx, [rbp - 1]
	add rax, rbx
	;; will lock rax. first = 26. second = [rbp - 1]. Locked: [rax, rbx]
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 342 ./examples/count_char_occurances_in_file.cy:16:32 Op(Minus) >
	mov rcx, 1
	mov rbx, rax
	sub rbx, rcx
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 4], bl
	
	
	mov QWORD [rbp - 24], 34
	mov QWORD [rbp - 32], string_0
	
	
	mov rax, 2
	
	mov rbx, [rbp - 32]
	
	mov rdi, rbx
	
	
	mov rsi, 0
	
	
	mov rdx, 0
	
	syscall
	
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 10
	syscall
	
	;; src/asm/internal_functions.rs:327
	mov rax, [rbp - 40]
	call _printRAX
	
	
	mov rax, 0
	
	mov rdi, [rbp - 40]
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, file
	
	mov rsi, rbx
	
	
	mov rdx, 4096
	
	syscall
	
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], rbx
	
	
	mov rax, 1
	
	
	mov rdi, 1
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, file
	
	mov rsi, rbx
	
	mov rdx, [rbp - 48]
	
	syscall
	
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 56], 0
	
	
	
	;; loop_0 start
	mov QWORD [rbp - 80], 1 ;; step
	mov rax, [rbp - 48] ;; to
	mov QWORD [rbp - 72], rax ;; to
	mov QWORD [rbp - 64], 0 ;; from
	.loop_0:
	
	;; Global new thingy
	xor rax, rax
	mov rax, file
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 56]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 56]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:239 assign_local_pointer var 'thing' of type Integer. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 88], rax
	
	mov rbx, [rbp - 88]
	xor rax, rax
	mov al, [rbx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov cl, [rbp - 1]
	mov bl, al
	cmp bl, cl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovge rax, rbx
	
	mov rcx, [rbp - 88]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov dl, [rbp - 2]
	mov cl, bl
	cmp cl, dl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
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
	
	mov rbx, [rbp - 88]
	xor rax, rax
	mov al, [rbx]
	
	;; Minus get the two operands from the stack. Result type: Integer8. Token: < 956 ./examples/count_char_occurances_in_file.cy:39:42 Op(Minus) >
	mov cl, [rbp - 1]
	mov bl, al
	sub bl, cl
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 89], bl
	
	;; Global new thingy
	xor rax, rax
	mov rax, lower_char_occurances
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 8
	mov rcx, [rbp - 89]
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = rax. second = rbx. Locked: [rbx, rax, rcx, rdx]
	
	;; src/asm/variable_assignment.rs:239 assign_local_pointer var 'addr_to_update' of type Integer. times_dereferenced: 0
	mov rax, rcx
	mov [rbp - 104], rax
	
	;; Dereferencing variable addr_to_update. handle_local_ptr_int_float
	mov rax, [rbp - 104]
	mov rax, [rax]
	xor rbx, rbx
	mov rbx, rax
	
	
	;; Plus get the two operands from the stack
	mov rax, 1
	mov rcx, rbx
	add rax, rcx
	;; will lock rax. first = 1. second = rbx. Locked: [rbx, rax, rcx]
	
	;; src/asm/variable_assignment.rs:239 assign_local_pointer var 'addr_to_update' of type Integer. times_dereferenced: 1
	mov rbx, rax
	mov rcx, [rbp - 104]
	mov [rcx], rbx
	
	jmp .if_end_0
	.if_end_0:
	
	
	mov rax, [rbp - 56]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 56], rax
	
	.loop_0_end_start:
	;; check exit condition
	mov rcx, [rbp - 80] ;; step
	mov rbx, [rbp - 72] ;; to
	mov rax, [rbp - 64] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	mov [rbp - 64], rax
	jmp .loop_0
	.loop_end_0:
	
	;; assign_local_number of type Integer
	mov rax, [rbp - 1]
	mov QWORD [rbp - 56], rax
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 112], 0
	
	
	
	
	;; loop_1 start
	mov QWORD [rbp - 136], 1 ;; step
	mov QWORD [rbp - 128], 26 ;; to
	mov QWORD [rbp - 120], 0 ;; from
	.loop_1:
	
	;; src/asm/internal_functions.rs:327
	mov rax, [rbp - 56]
	call _printRAX
	
	;; Global new thingy
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
	mov rcx, [rcx]
	
	;; get_vec_for_write_number. stack_member: rcx
	xor rax, rax
	mov rax, rcx
	call _printRAX
	
	
	mov rax, [rbp - 112]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 112], rax
	
	
	mov rax, [rbp - 56]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 56], rax
	
	.loop_1_end_start:
	;; check exit condition
	mov rcx, [rbp - 136] ;; step
	mov rbx, [rbp - 128] ;; to
	mov rax, [rbp - 120] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_1
	mov [rbp - 120], rax
	jmp .loop_1
	.loop_end_1:
	
	mov rsp, rbp
	pop rbp
	ret
	

