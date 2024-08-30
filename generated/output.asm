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
	
	pop rax
	mov [READ_SYSCALL], rax
	
	
	pop rax
	mov [WRITE_SYSCALL], rax
	
	
	pop rax
	mov [OPEN_SYSCALL], rax
	
	
	pop rax
	mov [CLOSE_SYSCALL], rax
	
	
	pop rax
	mov [MMAP_SYSCALL], rax
	
	
	pop rax
	mov [SOCKET_SYSCALL], rax
	
	
	pop rax
	mov [ACCEPT_SYSCALL], rax
	
	
	pop rax
	mov [BIND_SYSCALL], rax
	
	
	pop rax
	mov [LISTEN_SYSCALL], rax
	
	
	pop rax
	mov [AF_INET], rax
	
	
	pop rax
	mov [S_ADDR], rax
	
	
	pop rax
	mov [PORT], rax
	
	
	pop rax
	mov [PAD], rax
	
	
	pop rax
	mov [SOCK_STREAM], rax
	
	
	pop rax
	mov [STDOUT], rax
	
	
	pop rax
	mov [STDIN], rax
	
	
	pop rax
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
	
	
	;; ShiftRight get the two operands from the stack
	xor rax, rax
	xor rcx, rcx
	pop rcx
	pop rax
	;; We can only shift left or right by 8 bits
	shr rax, cl
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 1
	mov rax, rax
	cmp rax, rbx
	sete al
	
	.if_0:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_0
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 8]
	sub rax, rbx
	
	;; gen_logical_statement
	xor rax, rax
	pop rax
	not rax
	push rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 16], rax
	
	
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
	mov QWORD [rbp - 16], [rbp - 8]
	
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
	
	
	
	mov QWORD [rbp - 56], 1
	mov QWORD [rbp - 48], [rbp - 16]
	mov QWORD [rbp - 40], 0
	.loop_0:
	
	mov rax, [rbp - 8]
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, 63
	mov rax, [rbp - 32]
	add rax, rbx
	push rax
	
	;; assign_local_pointer of type Integer
	pop rax
	mov [rbp - 64], rax
	
	;; assign_local_pointer of type Integer
	pop rax
	mov rbx, [rbp - 64]
	mov [rbx], rax
	
	
	mov rax, [rbp - 32]
	pop rbx
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
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, 8
	mov rax, [rbp - 16]
	add rax, rbx
	;; binary op ptr -> char
	mov rbx, rax
	xor rax, rax
	mov al, [rbx]
	push rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, rax
	cmp rax, rbx
	sete al
	
	.if_1:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_1
	
	;; --- break ----
	jmp .loop_end_1
	
	jmp .if_end_1
	.if_end_1:
	
	
	mov rax, [rbp - 16]
	pop rbx
	add rax, rbx
	mov [rbp - 16], rax
	
	.loop_1_end_start:
	jmp .loop_1
	.loop_end_1:
	
	pop rax
	
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
	push rax
	
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 16]
	mov rax, 8
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type String
	pop rax
	mov [rbp - 16], rax
	
	;; Dereferencing variable len
	mov rbx, [rbp - 16]
	mov rbx, [rbx]
	xor rax, rax
	mov rax, rbx
	push rax
	;; Finish dereferencing variable len
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], rax
	
	pop rax
	
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
	
	
	
	mov rax, [rbp - 24]
	push rax
	
	mov rax, [OPEN_SYSCALL]
	push rax
	
	pop rax
	pop rdi
	pop rsi
	pop rdx
	syscall
	push rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], 0
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 45
	syscall
	
	pop rdi
	call _print_int
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 32]
	cmp rax, rbx
	setl al
	
	.if_2:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_2
	
	
	pop rax
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_2
	.if_end_2:
	
	mov rax, [rbp - 8]
	push rax
	
	
	pop rax
	pop rdi
	pop rsi
	pop rdx
	syscall
	push rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], 0
	
	mov rax, [CLOSE_SYSCALL]
	push rax
	
	pop rax
	pop rdi
	syscall
	push rax
	
	pop rax
	
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
	mov QWORD [rbp - 32], [rbp - 16]
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], 0
	
	.loop_2:
	
	
	;; Divide clean up rdx as this might mess up the final output as 'div' divides rdx:rax with the register we pass to div inst
	xor rdx, rdx
	;; get the two operands from the stack
	mov rbx, 10
	mov rax, [rbp - 32]
	div rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], rax
	
	
	mov rax, [rbp - 40]
	pop rbx
	add rax, rbx
	mov [rbp - 40], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 32]
	cmp rax, rbx
	sete al
	
	.if_3:
	cmp al, 0
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
	mov QWORD [rbp - 32], [rbp - 16]
	
	.loop_3:
	
	
	;; Modulo get the two operands from the stack
	xor rdx, rdx
	mov rbx, 10
	mov rax, [rbp - 32]
	div rbx
	mov rax, rdx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 56], rax
	
	mov rax, [rbp - 8]
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 48]
	add rax, rbx
	push rax
	
	;; assign_local_pointer of type Integer8
	pop rax
	mov [rbp - 64], rax
	
	;; Plus get the two operands from the stack
	mov rbx, [rbp - 56]
	mov rax, [rbp - 24]
	add rax, rbx
	
	;; assign_local_pointer of type Integer8
	pop rax
	mov rbx, [rbp - 64]
	mov [rbx], al
	
	
	;; Divide clean up rdx as this might mess up the final output as 'div' divides rdx:rax with the register we pass to div inst
	xor rdx, rdx
	;; get the two operands from the stack
	mov rbx, 10
	mov rax, [rbp - 32]
	div rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], rax
	
	
	mov rax, [rbp - 48]
	pop rbx
	sub rax, rbx
	mov [rbp - 48], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 32]
	cmp rax, rbx
	sete al
	
	.if_4:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_4
	
	;; --- break ----
	jmp .loop_end_3
	
	jmp .if_end_4
	.if_end_4:
	
	.loop_3_end_start:
	jmp .loop_3
	.loop_end_3:
	
	pop rax
	
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
	setg al
	
	.if_5:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_5
	
	
	pop rax
	
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
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, 0
	mov rax, [rbp - 40]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rax, [rbp - 48]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 64], rax
	
	mov rbx, [rbp - 56]
	xor rax, rax
	mov al, [rbx]
	push rax
	
	mov rbx, [rbp - 64]
	xor rax, rax
	mov al, [rbx]
	push rax
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, rax
	mov rax, [rbp - 40]
	cmp rax, rbx
	setne al
	
	.if_6:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_6
	
	
	pop rax
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_6
	.if_end_6:
	
	
	mov rax, [rbp - 40]
	pop rbx
	sub rax, rbx
	mov [rbp - 40], rax
	
	
	mov rax, [rbp - 48]
	pop rbx
	sub rax, rbx
	mov [rbp - 48], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 48]
	cmp rax, rbx
	setl al
	
	.if_7:
	cmp al, 0
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
	mov rbx, [rbp - 40]
	mov rax, 1
	add rax, rbx
	
	pop rax
	
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
	setg al
	
	.if_8:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_8
	
	
	pop rax
	
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
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, 0
	mov rax, [rbp - 40]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rax, [rbp - 48]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 64], rax
	
	mov rbx, [rbp - 56]
	xor rax, rax
	mov al, [rbx]
	push rax
	
	mov rbx, [rbp - 64]
	xor rax, rax
	mov al, [rbx]
	push rax
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, rax
	mov rax, rax
	cmp rax, rbx
	setne al
	
	.if_9:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_9
	
	
	pop rax
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_9
	.if_end_9:
	
	
	mov rax, [rbp - 40]
	pop rbx
	add rax, rbx
	mov [rbp - 40], rax
	
	
	mov rax, [rbp - 48]
	pop rbx
	add rax, rbx
	mov [rbp - 48], rax
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 32]
	sub rax, rbx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, rax
	mov rax, [rbp - 48]
	cmp rax, rbx
	setge al
	
	.if_10:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_10
	
	;; --- break ----
	jmp .loop_end_5
	
	jmp .if_end_10
	.if_end_10:
	
	.loop_5_end_start:
	jmp .loop_5
	.loop_end_5:
	
	
	pop rax
	
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
	setne al
	
	.if_11:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_11
	
	
	pop rax
	
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
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, 0
	mov rax, [rbp - 40]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rax, [rbp - 48]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 64], rax
	
	mov rbx, [rbp - 56]
	xor rax, rax
	mov al, [rbx]
	push rax
	
	mov rbx, [rbp - 64]
	xor rax, rax
	mov al, [rbx]
	push rax
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, rax
	mov rax, 1
	cmp rax, rbx
	setne al
	
	.if_12:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_12
	
	
	pop rax
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_12
	.if_end_12:
	
	
	mov rax, [rbp - 40]
	pop rbx
	add rax, rbx
	mov [rbp - 40], rax
	
	
	mov rax, [rbp - 48]
	pop rbx
	add rax, rbx
	mov [rbp - 48], rax
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 16]
	sub rax, rbx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, rax
	mov rax, [rbp - 48]
	cmp rax, rbx
	setge al
	
	.if_13:
	cmp al, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_13
	
	;; --- break ----
	jmp .loop_end_6
	
	jmp .if_end_13
	.if_end_13:
	
	.loop_6_end_start:
	jmp .loop_6
	.loop_end_6:
	
	
	pop rax
	
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
	
	
	
	mov QWORD [rbp - 72], 1
	mov QWORD [rbp - 64], [rbp - 16]
	mov QWORD [rbp - 56], 0
	mov QWORD [rbp - 80], 0
	.loop_7:
	
	mov rax, [rbp - 40]
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 48]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 88], rax
	
	mov rax, [rbp - 8]
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rax, [rbp - 80]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 96], rax
	
	mov rbx, [rbp - 96]
	xor rax, rax
	mov al, [rbx]
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov rbx, [rbp - 88]
	mov [rbx], al
	
	
	mov rax, [rbp - 48]
	pop rbx
	add rax, rbx
	mov [rbp - 48], rax
	
	.loop_7_end_start:
	;; inc the loop variable
	mov rdx, [rbp - 80]
	mov rcx, [rbp - 72]
	add rdx, rcx
	mov [rbp - 80], rdx
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
	
	
	
	mov QWORD [rbp - 120], 1
	mov QWORD [rbp - 112], [rbp - 32]
	mov QWORD [rbp - 104], 0
	mov QWORD [rbp - 128], 0
	.loop_8:
	
	mov rax, [rbp - 40]
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, 1
	mov rax, [rbp - 48]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 136], rax
	
	mov rax, [rbp - 24]
	push rax
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rax, [rbp - 128]
	add rax, rbx
	;; binary op ptr -> char
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 144], rax
	
	mov rbx, [rbp - 144]
	xor rax, rax
	mov al, [rbx]
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov rbx, [rbp - 136]
	mov [rbx], al
	
	
	mov rax, [rbp - 48]
	pop rbx
	add rax, rbx
	mov [rbp - 48], rax
	
	.loop_8_end_start:
	;; inc the loop variable
	mov rdx, [rbp - 128]
	mov rcx, [rbp - 120]
	add rdx, rcx
	mov [rbp - 128], rdx
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
	
	pop rax
	
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
	push rax
	
	pop rdi
	call _strlen
	push rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], [rbp - 48]
	
	
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
	push rax
	
	pop rdi
	call _strlen
	push rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], 1
	
	mov rax, [rbp - 40]
	push rax
	
	pop rdi
	call _strlen_cstr
	push rax
	
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
	

