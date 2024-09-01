%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	READ_SYSCALL dq 0
	WRITE_SYSCALL dq 0
	OPEN_SYSCALL dq 0
	CLOSE_SYSCALL dq 0
	MMAP_SYSCALL dq 0
	SOCKET_SYSCALL dq 0
	ACCEPT_SYSCALL dq 0
	BIND_SYSCALL dq 0
	LISTEN_SYSCALL dq 0
	AF_INET dq 0
	S_ADDR dq 0
	PORT dq 0
	PAD dq 0
	SOCK_STREAM dq 0
	STDOUT dq 0
	STDIN dq 0
	STDERR dq 0
	string_0 db 45
	string_1 db 115,121,115,99,97,108,108,40,79,80,69,78,95,83,89,83,67,65,76,76,44,32,97,98,115,95,102,105,108,101,95,112,97,116,104,44,32,48,44,32,48,41,32,61,32
	string_2 db 104,101,108,108,111,95,119,111,114,108,100
	string_3 db 108,101,110,32,115,116,114,105,110,103,32,61,32
	string_4 db 104,101,108,108,111,95,119,111,114,108,100,0
	string_5 db 108,101,110,49,32,61,32
	string_6 db 108,101,110,50,32,61,32

section .text
	global _start

_start:
	mov [argc], rsp
	
	mov rax, 0
	mov [READ_SYSCALL], rax
	
	
	mov rax, 1
	mov [WRITE_SYSCALL], rax
	
	
	mov rax, 2
	mov [OPEN_SYSCALL], rax
	
	
	mov rax, 3
	mov [CLOSE_SYSCALL], rax
	
	
	mov rax, 9
	mov [MMAP_SYSCALL], rax
	
	
	mov rax, 41
	mov [SOCKET_SYSCALL], rax
	
	
	mov rax, 43
	mov [ACCEPT_SYSCALL], rax
	
	
	mov rax, 49
	mov [BIND_SYSCALL], rax
	
	
	mov rax, 50
	mov [LISTEN_SYSCALL], rax
	
	
	mov rax, 2
	mov [AF_INET], rax
	
	
	mov rax, 16777343
	mov [S_ADDR], rax
	
	
	mov rax, 34835
	mov [PORT], rax
	
	
	mov rax, 0
	mov [PAD], rax
	
	
	mov rax, 1
	mov [SOCK_STREAM], rax
	
	
	mov rax, 1
	mov [STDOUT], rax
	
	
	mov rax, 0
	mov [STDIN], rax
	
	
	mov rax, 2
	mov [STDERR], rax
	
	call _main
	
	exit 0

_print_int:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 16
	;; param name a
	mov [rbp - 8], rdi
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 16], 0
	
	
	;; ShiftLeft get the two operands from the stack
	xor rax, rax
	xor rcx, rcx
	mov rax, 63
	mov rcx, [rbp - 8]
	;; We can only shift left or right by 8 bits
	shr rax, cl
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 1
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_0:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_0
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 8]
	sub rax, rbx
	
	not rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 16], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_0
	mov rdx, 1
	syscall
	
	mov rax, QWORD [rbp - 16]
	call _printRAX
	
	jmp .else_end_0
	.if_end_0:
	
	.else_0:
	
	;; assign_local_number of type Integer
	mov rax, [rbp - 8]
	mov QWORD [rbp - 16], rax
	
	mov rax, QWORD [rbp - 16]
	call _printRAX
	
	.else_end_0:
	
	mov rsp, rbp
	pop rbp
	ret
	

_memset:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 64
	;; param name ptr
	mov [rbp - 8], rdi
	;; param name value
	mov [rbp - 16], rsi
	;; param name size
	mov [rbp - 24], rdx
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], 0
	
	
	
	;; loop_0 start
	mov QWORD [rbp - 56], 1 ;; step
	mov rax, [rbp - 16] ;; to
	mov QWORD [rbp - 48], rax ;; to
	mov QWORD [rbp - 40], 0 ;; from
	.loop_0:
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 32]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 32]. second = rax. Locked: [rax, rbx, rcx]
	
	;; assign_local_pointer of type Integer
	mov rax, rbx
	mov [rbp - 64], rax
	
	;; assign_local_pointer of type Integer
	mov rax, [rbp - 16]
	mov rbx, [rbp - 64]
	mov [rbx], rax
	
	
	mov rax, [rbp - 32]
	mov rbx, 8
	add rax, rbx
	mov [rbp - 32], rax
	
	.loop_0_end_start:
	;; check exit condition
	mov rcx, [rbp - 56] ;; step
	mov rbx, [rbp - 48] ;; to
	mov rax, [rbp - 40] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	mov [rbp - 40], rax
	jmp .loop_0
	.loop_end_0:
	
	mov rsp, rbp
	pop rbp
	ret
	

_strlen_cstr:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 48
	;; param name string
	mov [rbp - 8], rdi
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 16], 0
	
	.loop_1:
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 16]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 16]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	mov rax, rbx
	xor rbx, rbx
	xor bl, [rax]
	
	
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
	je .if_end_1
	
	;; --- break ----
	jmp .loop_end_1
	
	jmp .if_end_1
	.if_end_1:
	
	
	mov rax, [rbp - 16]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 16], rax
	
	.loop_1_end_start:
	jmp .loop_1
	.loop_end_1:
	
	mov rax, [rbp - 16]
	
	mov rsp, rbp
	pop rbp
	ret
	
	mov rsp, rbp
	pop rbp
	ret
	

_strlen:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 32
	;; param name string
	mov [rbp - 8], rdi
	
	mov rax, [rbp - 8]
	
	
	;; Plus get the two operands from the stack
	mov rbx, 8
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 8. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> str
	
	;; assign_local_pointer of type String
	mov rax, rbx
	mov [rbp - 16], rax
	
	;; Dereferencing variable len
	mov rbx, [rbp - 16]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], rax
	
	mov rax, [rbp - 24]
	
	mov rsp, rbp
	pop rbp
	ret
	
	mov rsp, rbp
	pop rbp
	ret
	

_read_file_into_memory:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 48
	;; param name memory
	mov [rbp - 8], rdi
	;; param name mem_size
	mov [rbp - 16], rsi
	;; param name abs_file_path
	mov [rbp - 24], rdx
	
	mov rax, [OPEN_SYSCALL]
	
	mov rax, rax
	
	mov rbx, [rbp - 24]
	
	mov rdi, rbx
	
	
	mov rsi, 0
	
	
	mov rdx, 0
	
	syscall
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 45
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 32]
	
	call _print_int
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 32]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_2:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_2
	
	
	mov rax, -1
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_2
	.if_end_2:
	
	
	mov rax, 0
	
	mov rdi, [rbp - 32]
	
	mov rbx, [rbp - 8]
	
	mov rsi, rbx
	
	mov rdx, [rbp - 16]
	
	syscall
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], rax
	
	mov rax, [CLOSE_SYSCALL]
	
	mov rax, rax
	
	mov rdi, [rbp - 32]
	
	syscall
	
	mov rax, [rbp - 40]
	
	mov rsp, rbp
	pop rbp
	ret
	
	mov rsp, rbp
	pop rbp
	ret
	

_write_int_into_mem:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 112
	;; param name memory
	mov [rbp - 8], rdi
	;; param name number
	mov [rbp - 16], rsi
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], 48
	
	;; assign_local_number of type Integer
	mov rax, [rbp - 16]
	mov QWORD [rbp - 32], rax
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], 0
	
	.loop_2:
	
	
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rbx, 10
	mov rax, [rbp - 32]
	div rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], rax
	
	
	mov rax, [rbp - 40]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 40], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 32]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_3:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_3
	
	;; --- break ----
	jmp .loop_end_2
	
	jmp .if_end_3
	.if_end_3:
	
	.loop_2_end_start:
	jmp .loop_2
	.loop_end_2:
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 40]
	sub rax, rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], rax
	
	;; assign_local_number of type Integer
	mov rax, [rbp - 16]
	mov QWORD [rbp - 32], rax
	
	.loop_3:
	
	
	;; Modulo get the two operands from the stack
	xor rdx, rdx
	mov rbx, 10
	mov rax, [rbp - 32]
	div rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 56], rdx
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	
	;; assign_local_pointer of type Integer8
	mov rax, rbx
	mov [rbp - 64], rax
	
	;; Plus get the two operands from the stack
	mov rax, [rbp - 24]
	mov rbx, [rbp - 56]
	add rax, rbx
	;; will lock rax. first = [rbp - 24]. second = [rbp - 56]. Locked: [rax, rbx]
	
	;; assign_local_pointer of type Integer8
	mov rbx, rax
	mov rcx, [rbp - 64]
	mov [rcx], bl
	
	
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rbx, 10
	mov rax, [rbp - 32]
	div rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], rax
	
	
	mov rax, [rbp - 48]
	mov rbx, 1
	sub rax, rbx
	mov [rbp - 48], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 32]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_4:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_4
	
	;; --- break ----
	jmp .loop_end_3
	
	jmp .if_end_4
	.if_end_4:
	
	.loop_3_end_start:
	jmp .loop_3
	.loop_end_3:
	
	mov rax, [rbp - 40]
	
	mov rsp, rbp
	pop rbp
	ret
	
	mov rsp, rbp
	pop rbp
	ret
	

_string_ends_with:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 96
	;; param name string
	mov [rbp - 8], rdi
	;; param name string_len
	mov [rbp - 16], rsi
	;; param name substr
	mov [rbp - 24], rdx
	;; param name substr_len
	mov [rbp - 32], rcx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, [rbp - 16]
	mov rax, [rbp - 32]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovg rax, rbx
	
	.if_5:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_5
	
	
	mov rax, 0
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_5
	.if_end_5:
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 16]
	sub rax, rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], rax
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 32]
	sub rax, rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], rax
	
	.loop_4:
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 40]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 40]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 64], rax
	
	mov rbx, [rbp - 56]
	xor rax, rax
	mov al, [rbx]
	
	mov rcx, [rbp - 64]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rbx
	mov rcx, rax
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovne rax, rbx
	
	.if_6:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_6
	
	
	mov rax, 0
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_6
	.if_end_6:
	
	
	mov rax, [rbp - 40]
	mov rbx, 1
	sub rax, rbx
	mov [rbp - 40], rax
	
	
	mov rax, [rbp - 48]
	mov rbx, 1
	sub rax, rbx
	mov [rbp - 48], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 48]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_7:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_7
	
	;; --- break ----
	jmp .loop_end_4
	
	jmp .if_end_7
	.if_end_7:
	
	.loop_4_end_start:
	jmp .loop_4
	.loop_end_4:
	
	
	;; Plus get the two operands from the stack
	mov rax, 1
	mov rbx, [rbp - 40]
	add rax, rbx
	;; will lock rax. first = 1. second = [rbp - 40]. Locked: [rax, rbx]
	
	mov rax, rax
	
	mov rsp, rbp
	pop rbp
	ret
	
	mov rsp, rbp
	pop rbp
	ret
	

_string_starts_with:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 96
	;; param name string
	mov [rbp - 8], rdi
	;; param name string_len
	mov [rbp - 16], rsi
	;; param name substr
	mov [rbp - 24], rdx
	;; param name substr_len
	mov [rbp - 32], rcx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, [rbp - 16]
	mov rax, [rbp - 32]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovg rax, rbx
	
	.if_8:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_8
	
	
	mov rax, 0
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_8
	.if_end_8:
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], 0
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], 0
	
	.loop_5:
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 40]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 40]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 64], rax
	
	mov rbx, [rbp - 56]
	xor rax, rax
	mov al, [rbx]
	
	mov rcx, [rbp - 64]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rbx
	mov rcx, rax
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovne rax, rbx
	
	.if_9:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_9
	
	
	mov rax, 0
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_9
	.if_end_9:
	
	
	mov rax, [rbp - 40]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 40], rax
	
	
	mov rax, [rbp - 48]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 48], rax
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 32]
	sub rax, rbx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, rax
	mov rbx, [rbp - 48]
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovge rax, rbx
	
	.if_10:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_10
	
	;; --- break ----
	jmp .loop_end_5
	
	jmp .if_end_10
	.if_end_10:
	
	.loop_5_end_start:
	jmp .loop_5
	.loop_end_5:
	
	
	mov rax, 1
	
	mov rsp, rbp
	pop rbp
	ret
	
	mov rsp, rbp
	pop rbp
	ret
	

_string_eq:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 96
	;; param name string
	mov [rbp - 8], rdi
	;; param name string_len
	mov [rbp - 16], rsi
	;; param name string2
	mov [rbp - 24], rdx
	;; param name string2_len
	mov [rbp - 32], rcx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, [rbp - 32]
	mov rax, [rbp - 16]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovne rax, rbx
	
	.if_11:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_11
	
	
	mov rax, 0
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_11
	.if_end_11:
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], 0
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], 0
	
	.loop_6:
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 40]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 40]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 64], rax
	
	mov rbx, [rbp - 56]
	xor rax, rax
	mov al, [rbx]
	
	mov rcx, [rbp - 64]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rbx
	mov rcx, rax
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovne rax, rbx
	
	.if_12:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_12
	
	
	mov rax, 0
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_12
	.if_end_12:
	
	
	mov rax, [rbp - 40]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 40], rax
	
	
	mov rax, [rbp - 48]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 48], rax
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 16]
	sub rax, rbx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, rax
	mov rbx, [rbp - 48]
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovge rax, rbx
	
	.if_13:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_13
	
	;; --- break ----
	jmp .loop_end_6
	
	jmp .if_end_13
	.if_end_13:
	
	.loop_6_end_start:
	jmp .loop_6
	.loop_end_6:
	
	
	mov rax, 1
	
	mov rsp, rbp
	pop rbp
	ret
	
	mov rsp, rbp
	pop rbp
	ret
	

_str_concat:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 144
	;; param name string
	mov [rbp - 8], rdi
	;; param name string_len
	mov [rbp - 16], rsi
	;; param name string2
	mov [rbp - 24], rdx
	;; param name string2_len
	mov [rbp - 32], rcx
	;; param name new_str
	mov [rbp - 40], r8
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], 0
	
	
	
	;; loop_7 start
	mov QWORD [rbp - 72], 1 ;; step
	mov rax, [rbp - 16] ;; to
	mov QWORD [rbp - 64], rax ;; to
	mov QWORD [rbp - 56], 0 ;; from
	mov QWORD [rbp - 80], 0 ;; loop variable i
	.loop_7:
	
	mov rax, [rbp - 40]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 88], rax
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 80]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 80]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 96], rax
	
	mov rbx, [rbp - 96]
	xor rax, rax
	mov al, [rbx]
	
	;; assign_local_pointer of type Character
	mov rbx, rax
	mov rcx, [rbp - 88]
	mov [rcx], bl
	
	
	mov rax, [rbp - 48]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 48], rax
	
	.loop_7_end_start:
	;; inc the loop variable
	mov rax, [rbp - 80]
	mov rbx, [rbp - 72]
	add rax, rbx
	mov [rbp - 80], rax
	;; check exit condition
	mov rcx, [rbp - 72] ;; step
	mov rbx, [rbp - 64] ;; to
	mov rax, [rbp - 56] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_7
	mov [rbp - 56], rax
	jmp .loop_7
	.loop_end_7:
	
	
	
	;; loop_8 start
	mov QWORD [rbp - 120], 1 ;; step
	mov rax, [rbp - 32] ;; to
	mov QWORD [rbp - 112], rax ;; to
	mov QWORD [rbp - 104], 0 ;; from
	mov QWORD [rbp - 128], 0 ;; loop variable i
	.loop_8:
	
	mov rax, [rbp - 40]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 136], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 128]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 128]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; assign_local_pointer of type Character
	mov rax, rbx
	mov [rbp - 144], rax
	
	mov rbx, [rbp - 144]
	xor rax, rax
	mov al, [rbx]
	
	;; assign_local_pointer of type Character
	mov rbx, rax
	mov rcx, [rbp - 136]
	mov [rcx], bl
	
	
	mov rax, [rbp - 48]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 48], rax
	
	.loop_8_end_start:
	;; inc the loop variable
	mov rax, [rbp - 128]
	mov rbx, [rbp - 120]
	add rax, rbx
	mov [rbp - 128], rax
	;; check exit condition
	mov rcx, [rbp - 120] ;; step
	mov rbx, [rbp - 112] ;; to
	mov rax, [rbp - 104] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_8
	mov [rbp - 104], rax
	jmp .loop_8
	.loop_end_8:
	
	mov rax, [rbp - 48]
	
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
	sub rsp, 64
	
	
	mov QWORD [rbp - 8], 11
	mov QWORD [rbp - 16], string_2
	
	lea rax, [rbp - 16]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_3
	mov rdx, 13
	syscall
	
	mov rax, QWORD [rbp - 24]
	call _printRAX
	
	
	mov QWORD [rbp - 32], 12
	mov QWORD [rbp - 40], string_4
	
	lea rax, [rbp - 40]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], rax
	
	mov rax, [rbp - 40]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen_cstr
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 56], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_5
	mov rdx, 7
	syscall
	
	mov rax, QWORD [rbp - 48]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_6
	mov rdx, 7
	syscall
	
	mov rax, QWORD [rbp - 56]
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	

