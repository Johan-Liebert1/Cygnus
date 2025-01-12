%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8
	clientaddr resb 1024
	read_data resb 4096
	serveraddr_mem resb 16
	clientaddr_mem resb 16
	file_len resb 32
	req_method resb 32
	req_path resb 256
	file_to_read resb 256

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
	S_ADDR dd 0
	PORT dw 0
	PAD dq 0
	SOCK_STREAM dq 0
	STDOUT dq 0
	STDIN dq 0
	STDERR dq 0
	string_0 db 45
	string_1 db 115,121,115,99,97,108,108,40,79,80,69,78,95,83,89,83,67,65,76,76,44,32,97,98,115,95,102,105,108,101,95,112,97,116,104,44,32,48,44,32,48,41,32,61,32
	PRINT_REQ dq 0
	SPACE_ASCII db 0
	NEW_LINE_ASCII db 0
	NULL_BYTE db 0
	string_2 db 87,114,105,116,105,110,103,32,114,101,113,32,116,111,32,115,116,100,111,117,116,46,46,46,10
	string_3 db 46,104,116,109,108
	string_4 db 72,84,84,80,47,49,46,49,32,50,48,48,32,79,75,13,10,67,111,110,116,101,110,116,45,84,121,112,101,58,32,116,101,120,116,47,112,108,97,105,110,13,10,67,111,110,116,101,110,116,45,76,101,110,103,116,104,58,32,50,13,10,13,10,79,75
	string_5 db 72,84,84,80,47,49,46,49,32,52,48,52,32,78,79,84,32,70,79,85,78,68,13,10,13,10
	string_6 db 72,84,84,80,47,49,46,49,32,53,48,48,32,73,110,116,101,114,110,97,108,32,83,101,114,118,101,114,32,69,114,114,111,114,13,10,13,10
	string_7 db 72,84,84,80,47,49,46,49,32,50,48,48,32,79,75,13,10,67,111,110,116,101,110,116,45,84,121,112,101,58,32,116,101,120,116,47,104,116,109,108,13,10,67,111,110,116,101,110,116,45,76,101,110,103,116,104,58,32
	string_8 db 13,10,13,10
	string_9 db 47,104,111,109,101,47,112,114,97,103,121,97,110,47,82,117,115,116,47,108,97,110,103,47,101,120,97,109,112,108,101,115,47,104,116,116,112,95,115,101,114,118,101,114
	string_10 db 109,101,116,104,111,100,95,101,110,100,115,95,97,116,95,105,100,120,32,61,32
	string_11 db 112,97,116,104,95,97,115,95,99,104,97,114,32,61,32
	string_12 db 87,114,105,116,105,110,103,32,104,116,116,112,95,52,48,52,32,116,111,32,99,111,110,110,102,100,32,114,101,116,117,114,110,101,100,58,32
	string_13 db 67,108,105,101,110,116,32,97,115,107,101,100,32,102,111,114,32,112,97,116,104,58,32
	string_14 db 10
	string_15 db 102,105,110,97,108,95,102,105,108,101,95,97,98,115,95,112,97,116,104,32,61,32
	string_16 db 10
	string_17 db 114,101,97,100,95,102,105,108,101,95,105,110,116,111,95,109,101,109,111,114,121,32,114,101,116,117,114,110,101,100,58,32
	string_18 db 82,101,97,100,32
	string_19 db 32,98,121,116,101,115,32,102,114,111,109,32,102,105,108,101,32
	string_20 db 10
	string_21 db 87,114,105,116,105,110,103,32,116,111,32,99,111,110,110,102,100,32,114,101,116,117,114,110,101,100,58,32
	string_22 db 87,114,105,116,105,110,103,32,104,101,97,100,101,114,95,98,111,100,121,95,115,101,112,101,114,97,116,111,114,32,116,111,32,99,111,110,110,102,100,32,114,101,116,117,114,110,101,100,58,32
	string_23 db 87,114,105,116,105,110,103,32,116,111,32,99,111,110,110,102,100,32,114,101,116,117,114,110,101,100,58,32
	string_24 db 83,79,67,75,69,84,95,83,89,83,67,65,76,76,32,114,101,116,117,114,110,58,32
	string_25 db 66,73,78,68,95,83,89,83,67,65,76,76,32,114,101,116,117,114,110,58,32
	string_26 db 76,73,83,84,69,78,95,83,89,83,67,65,76,76,32,114,101,116,117,114,110,58,32
	string_27 db 65,67,67,69,80,84,95,83,89,83,67,65,76,76,32,114,101,116,117,114,110,58,32
	string_28 db 82,101,97,100,32
	string_29 db 32,98,121,116,101,115,32,10

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
	;; param name a. Param type Integer
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
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_0:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_0
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 564 ./examples/http_server/../include/std.cy:26:18 Op(Minus) >
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
	
	;; src/asm/internal_functions.rs:320
	mov rax, [rbp - 16]
	call _printRAX
	
	jmp .else_end_0
	.if_end_0:
	
	.else_0:
	;; assign_local_number of type Integer
	mov rax, [rbp - 8]
	mov QWORD [rbp - 16], rax
	
	;; src/asm/internal_functions.rs:320
	mov rax, [rbp - 16]
	call _printRAX
	
	.else_end_0:
	;; 'a' at '[rbp - 8]'
	;; 'n' at '[rbp - 16]'
	mov rsp, rbp
	pop rbp
	ret
	

_memset:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 64
	;; param name ptr. Param type Pointer -> Integer
	mov [rbp - 8], rdi
	;; param name value. Param type Integer
	mov [rbp - 16], rsi
	;; param name size. Param type Integer
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
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 768 ./examples/http_server/../include/std.cy:38:28 Op(Plus) >
	mov rbx, [rbp - 32]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 32]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'thing' of type Integer. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 64], rax
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'thing' of type Integer. times_dereferenced: 1
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
	
	;; 'size' at '[rbp - 24]'
	;; 'ptr' at '[rbp - 8]'
	;; 'loop_0_to' at '[rbp - 48]'
	;; 'i' at '[rbp - 32]'
	;; 'loop_0_step' at '[rbp - 56]'
	;; 'value' at '[rbp - 16]'
	;; 'loop_0_from' at '[rbp - 40]'
	mov rsp, rbp
	pop rbp
	ret
	

_strlen_cstr:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 48
	;; param name string. Param type Pointer -> Character
	mov [rbp - 8], rdi
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 16], 0
	
	.loop_1:
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 915 ./examples/http_server/../include/std.cy:49:19 Op(Plus) >
	mov rbx, [rbp - 16]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 16]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	mov rax, rbx
	xor rbx, rbx
	xor bl, [rax]
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov cl, 0
	mov al, bl
	cmp al, cl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
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
	
	;; Returning from function
	mov rax, [rbp - 16]
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'string' at '[rbp - 8]'
	;; 'i' at '[rbp - 16]'

_strlen:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 32
	;; param name string. Param type Pointer -> String
	mov [rbp - 8], rdi
	
	mov rax, [rbp - 8]
	
	
	;; Plus get the two operands from the stack. Result type: Pointer -> String. Token: < 1058 ./examples/http_server/../include/std.cy:60:25 Op(Plus) >
	mov rbx, 8
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 8. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> str
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'len' of type String. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 16], rax
	
	;; Dereferencing variable len. handle_local_ptr_int_float
	mov rax, [rbp - 16]
	mov rax, [rax]
	xor rbx, rbx
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 24], rbx
	
	;; Returning from function
	mov rax, [rbp - 24]
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'length' at '[rbp - 24]'
	;; 'string' at '[rbp - 8]'
	;; 'len' at '[rbp - 16]'

_read_file_into_memory:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 48
	;; param name memory. Param type Pointer -> Integer
	mov [rbp - 8], rdi
	;; param name mem_size. Param type Integer
	mov [rbp - 16], rsi
	;; param name abs_file_path. Param type Pointer -> Character
	mov [rbp - 24], rdx
	
	mov rax, QWORD [OPEN_SYSCALL]
	
	mov rbx, [rbp - 24]
	
	mov rdi, rbx
	
	
	mov rsi, 0
	
	
	mov rdx, 0
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], rbx
	
	
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
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_2:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_2
	
	
	;; Returning from function
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
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], rbx
	
	mov rax, QWORD [CLOSE_SYSCALL]
	
	mov rdi, [rbp - 32]
	
	syscall
	
	;; Returning from function
	mov rax, [rbp - 40]
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'fd' at '[rbp - 32]'
	;; 'abs_file_path' at '[rbp - 24]'
	;; 'mem_size' at '[rbp - 16]'
	;; 'read_bytes' at '[rbp - 40]'
	;; 'memory' at '[rbp - 8]'

_write_int_into_mem:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 112
	;; param name memory. Param type Pointer -> Integer
	mov [rbp - 8], rdi
	;; param name number. Param type Integer
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
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
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
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 1942 ./examples/http_server/../include/std.cy:101:29 Op(Minus) >
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
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 2044 ./examples/http_server/../include/std.cy:107:38 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'idx_into_mem' of type Integer. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 64], rax
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 2077 ./examples/http_server/../include/std.cy:108:25 Op(Plus) >
	mov rax, [rbp - 24]
	mov rbx, [rbp - 56]
	add rax, rbx
	;; will lock rax. first = [rbp - 24]. second = [rbp - 56]. Locked: [rax, rbx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'idx_into_mem' of type Integer. times_dereferenced: 1
	mov rbx, rax
	mov rcx, [rbp - 64]
	mov [rcx], rbx
	
	
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
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
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
	
	;; Returning from function
	mov rax, [rbp - 40]
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'number' at '[rbp - 16]'
	;; 'n' at '[rbp - 32]'
	;; 'idx' at '[rbp - 48]'
	;; 'memory' at '[rbp - 8]'
	;; 'idx_into_mem' at '[rbp - 64]'
	;; 'c' at '[rbp - 56]'
	;; 'zero_ascii' at '[rbp - 24]'
	;; 'number_len' at '[rbp - 40]'

_string_ends_with:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 96
	;; param name string. Param type Pointer -> Character
	mov [rbp - 8], rdi
	;; param name string_len. Param type Integer
	mov [rbp - 16], rsi
	;; param name substr. Param type Pointer -> Character
	mov [rbp - 24], rdx
	;; param name substr_len. Param type Integer
	mov [rbp - 32], rcx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, [rbp - 16]
	mov rax, [rbp - 32]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovg rax, rbx
	
	.if_5:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_5
	
	
	;; Returning from function
	mov rax, 0
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_5
	.if_end_5:
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 2508 ./examples/http_server/../include/std.cy:127:38 Op(Minus) >
	mov rbx, 1
	mov rax, [rbp - 16]
	sub rax, rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], rax
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 2555 ./examples/http_server/../include/std.cy:128:41 Op(Minus) >
	mov rbx, 1
	mov rax, [rbp - 32]
	sub rax, rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], rax
	
	.loop_4:
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 2609 ./examples/http_server/../include/std.cy:131:35 Op(Plus) >
	mov rbx, [rbp - 40]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 40]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'str_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 2665 ./examples/http_server/../include/std.cy:132:38 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'substr_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 64], rax
	
	mov rbx, [rbp - 56]
	xor rax, rax
	mov al, [rbx]
	
	mov rcx, [rbp - 64]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov dl, bl
	mov cl, al
	cmp cl, dl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovne rax, rbx
	
	.if_6:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_6
	
	
	;; Returning from function
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
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
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
	
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 2908 ./examples/http_server/../include/std.cy:146:25 Op(Plus) >
	mov rax, 1
	mov rbx, [rbp - 40]
	add rax, rbx
	;; will lock rax. first = 1. second = [rbp - 40]. Locked: [rax, rbx]
	
	;; Returning from function
	mov rax, rax
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'substr' at '[rbp - 24]'
	;; 'substr_char' at '[rbp - 64]'
	;; 'idx_into_substr' at '[rbp - 48]'
	;; 'substr_len' at '[rbp - 32]'
	;; 'str_char' at '[rbp - 56]'
	;; 'string' at '[rbp - 8]'
	;; 'idx_into_str' at '[rbp - 40]'
	;; 'string_len' at '[rbp - 16]'

_string_starts_with:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 96
	;; param name string. Param type Pointer -> Character
	mov [rbp - 8], rdi
	;; param name string_len. Param type Integer
	mov [rbp - 16], rsi
	;; param name substr. Param type Pointer -> Character
	mov [rbp - 24], rdx
	;; param name substr_len. Param type Integer
	mov [rbp - 32], rcx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, [rbp - 16]
	mov rax, [rbp - 32]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovg rax, rbx
	
	.if_8:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_8
	
	
	;; Returning from function
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
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 3230 ./examples/http_server/../include/std.cy:159:35 Op(Plus) >
	mov rbx, [rbp - 40]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 40]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'str_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 3286 ./examples/http_server/../include/std.cy:160:38 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'substr_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 64], rax
	
	mov rbx, [rbp - 56]
	xor rax, rax
	mov al, [rbx]
	
	mov rcx, [rbp - 64]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov dl, bl
	mov cl, al
	cmp cl, dl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovne rax, rbx
	
	.if_9:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_9
	
	
	;; Returning from function
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
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 3477 ./examples/http_server/../include/std.cy:169:40 Op(Minus) >
	mov rbx, 1
	mov rax, [rbp - 32]
	sub rax, rbx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, rax
	mov rbx, [rbp - 48]
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
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
	
	
	;; Returning from function
	mov rax, 1
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'substr_char' at '[rbp - 64]'
	;; 'str_char' at '[rbp - 56]'
	;; 'substr' at '[rbp - 24]'
	;; 'string' at '[rbp - 8]'
	;; 'substr_len' at '[rbp - 32]'
	;; 'string_len' at '[rbp - 16]'
	;; 'idx_into_str' at '[rbp - 40]'
	;; 'idx_into_substr' at '[rbp - 48]'

_string_eq:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 96
	;; param name string. Param type Pointer -> Character
	mov [rbp - 8], rdi
	;; param name string_len. Param type Integer
	mov [rbp - 16], rsi
	;; param name string2. Param type Pointer -> Character
	mov [rbp - 24], rdx
	;; param name string2_len. Param type Integer
	mov [rbp - 32], rcx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, [rbp - 32]
	mov rax, [rbp - 16]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovne rax, rbx
	
	.if_11:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_11
	
	
	;; Returning from function
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
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 3799 ./examples/http_server/../include/std.cy:186:35 Op(Plus) >
	mov rbx, [rbp - 40]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 40]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'str_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 3856 ./examples/http_server/../include/std.cy:187:39 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'substr_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 64], rax
	
	mov rbx, [rbp - 56]
	xor rax, rax
	mov al, [rbx]
	
	mov rcx, [rbp - 64]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov dl, bl
	mov cl, al
	cmp cl, dl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovne rax, rbx
	
	.if_12:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_12
	
	
	;; Returning from function
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
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 4047 ./examples/http_server/../include/std.cy:196:40 Op(Minus) >
	mov rbx, 1
	mov rax, [rbp - 16]
	sub rax, rbx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, rax
	mov rbx, [rbp - 48]
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
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
	
	
	;; Returning from function
	mov rax, 1
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'string_len' at '[rbp - 16]'
	;; 'string2_len' at '[rbp - 32]'
	;; 'string' at '[rbp - 8]'
	;; 'str_char' at '[rbp - 56]'
	;; 'idx_into_substr' at '[rbp - 48]'
	;; 'idx_into_str' at '[rbp - 40]'
	;; 'string2' at '[rbp - 24]'
	;; 'substr_char' at '[rbp - 64]'

_str_concat:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 144
	;; param name string. Param type Pointer -> Character
	mov [rbp - 8], rdi
	;; param name string_len. Param type Integer
	mov [rbp - 16], rsi
	;; param name string2. Param type Pointer -> Character
	mov [rbp - 24], rdx
	;; param name string2_len. Param type Integer
	mov [rbp - 32], rcx
	;; param name new_str. Param type Pointer -> Character
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
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4328 ./examples/http_server/../include/std.cy:208:39 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'new_str_idx' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 88], rax
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4385 ./examples/http_server/../include/std.cy:209:35 Op(Plus) >
	mov rbx, [rbp - 80]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 80]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'old_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 96], rax
	
	mov rbx, [rbp - 96]
	xor rax, rax
	mov al, [rbx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'new_str_idx' of type Character. times_dereferenced: 1
	mov bl, al
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
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4545 ./examples/http_server/../include/std.cy:217:39 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'new_str_idx' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 136], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4603 ./examples/http_server/../include/std.cy:218:36 Op(Plus) >
	mov rbx, [rbp - 128]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 128]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'old_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 144], rax
	
	mov rbx, [rbp - 144]
	xor rax, rax
	mov al, [rbx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'new_str_idx' of type Character. times_dereferenced: 1
	mov bl, al
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
	
	;; Returning from function
	mov rax, [rbp - 48]
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'string_len' at '[rbp - 16]'
	;; 'loop_7_to' at '[rbp - 64]'
	;; 'new_str' at '[rbp - 40]'
	;; 'loop_7_step' at '[rbp - 72]'
	;; 'loop_7_from' at '[rbp - 56]'
	;; 'loop_8_to' at '[rbp - 112]'
	;; 'loop_8_from' at '[rbp - 104]'
	;; 'loop_8_step' at '[rbp - 120]'
	;; 'idx_into_new_str' at '[rbp - 48]'
	;; 'string' at '[rbp - 8]'
	;; 'string2_len' at '[rbp - 32]'
	;; 'string2' at '[rbp - 24]'
	
	mov rax, 1
	mov [PRINT_REQ], rax
	
	
	mov rax, 32
	mov [SPACE_ASCII], rax
	
	
	mov rax, 10
	mov [NEW_LINE_ASCII], rax
	
	
	mov rax, 0
	mov [NULL_BYTE], rax
	

_parse_http_request:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 336
	;; param name connfd. Param type Integer
	mov [rbp - 8], rdi
	;; param name req. Param type Pointer -> Integer8
	mov [rbp - 16], rsi
	;; param name read_bytes. Param type Integer
	mov [rbp - 24], rdx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_2
	mov rdx, 25
	syscall
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	mov rbx, [rbp - 16]
	
	mov rsi, rbx
	
	mov rdx, [rbp - 24]
	
	syscall
	
	
	mov QWORD [rbp - 40], 5
	mov QWORD [rbp - 48], string_3
	
	
	mov QWORD [rbp - 56], 66
	mov QWORD [rbp - 64], string_4
	
	
	mov QWORD [rbp - 72], 26
	mov QWORD [rbp - 80], string_5
	
	
	mov QWORD [rbp - 88], 38
	mov QWORD [rbp - 96], string_6
	
	
	mov QWORD [rbp - 104], 58
	mov QWORD [rbp - 112], string_7
	
	lea rax, [rbp - 112]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 120], rax
	
	
	mov QWORD [rbp - 136], 4
	mov QWORD [rbp - 144], string_8
	
	lea rax, [rbp - 144]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 152], rax
	
	
	mov QWORD [rbp - 168], 44
	mov QWORD [rbp - 176], string_9
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 184], 0
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 192], 0
	
	.loop_9:
	mov rax, [rbp - 16]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer8. Token: < 1514 ./examples/http_server/index.cy:58:33 Op(Plus) >
	mov rbx, [rbp - 184]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 184]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'character' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 200], rax
	
	mov rbx, [rbp - 200]
	xor rax, rax
	mov al, [rbx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov cl, BYTE [SPACE_ASCII]
	mov bl, al
	cmp bl, cl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	mov rcx, [rbp - 200]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov dl, BYTE [NULL_BYTE]
	mov cl, bl
	cmp cl, dl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; gen_logical_statement
	xor rcx, rcx
	mov rcx, rbx
	xor rdx, rdx
	mov rdx, rax
	or rcx, rdx
	
	.if_14:
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_14
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 1625 ./examples/http_server/index.cy:61:37 Op(Minus) >
	mov rbx, 1
	mov rax, [rbp - 184]
	sub rax, rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 192], rax
	
	;; --- break ----
	jmp .loop_end_9
	
	jmp .if_end_14
	.if_end_14:
	
	
	mov rax, [rbp - 184]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 184], rax
	
	.loop_9_end_start:
	jmp .loop_9
	.loop_end_9:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_10
	mov rdx, 21
	syscall
	
	;; src/asm/internal_functions.rs:320
	mov rax, [rbp - 192]
	call _printRAX
	
	
	mov rax, [rbp - 184]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 184], rax
	
	;; assign_local_number of type Integer
	mov rax, [rbp - 184]
	mov QWORD [rbp - 208], rax
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 216], -1
	
	.loop_10:
	mov rax, [rbp - 16]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer8. Token: < 1932 ./examples/http_server/index.cy:78:34 Op(Plus) >
	mov rbx, [rbp - 184]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 184]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'character1' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 224], rax
	
	mov rbx, [rbp - 224]
	xor rax, rax
	mov al, [rbx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov cl, BYTE [SPACE_ASCII]
	mov bl, al
	cmp bl, cl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	mov rcx, [rbp - 224]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov dl, BYTE [NULL_BYTE]
	mov cl, bl
	cmp cl, dl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; gen_logical_statement
	xor rcx, rcx
	mov rcx, rbx
	xor rdx, rdx
	mov rdx, rax
	or rcx, rdx
	
	.if_15:
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_15
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 2043 ./examples/http_server/index.cy:81:35 Op(Minus) >
	mov rbx, 1
	mov rax, [rbp - 184]
	sub rax, rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 216], rax
	
	;; --- break ----
	jmp .loop_end_10
	
	jmp .if_end_15
	.if_end_15:
	
	
	mov rax, [rbp - 184]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 184], rax
	
	.loop_10_end_start:
	jmp .loop_10
	.loop_end_10:
	
	mov rax, [rbp - 16]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer8. Token: < 2137 ./examples/http_server/index.cy:88:32 Op(Plus) >
	mov rbx, [rbp - 208]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 208]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'path_as_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 232], rax
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 2200 ./examples/http_server/index.cy:89:40 Op(Minus) >
	mov rbx, [rbp - 208]
	mov rax, [rbp - 216]
	sub rax, rbx
	
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 2221 ./examples/http_server/index.cy:89:59 Op(Plus) >
	mov rbx, 1
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 1. second = rax. Locked: [rax, rbx, rcx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 240], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_11
	mov rdx, 15
	syscall
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	mov rbx, [rbp - 232]
	
	mov rsi, rbx
	
	mov rdx, [rbp - 240]
	
	syscall
	
	mov rax, [rbp - 232]
	
	;; Moving argument number 1
	mov rdi, rax
	
	;; Moving argument number 2
	mov rsi, [rbp - 240]
	
	mov rax, [rbp - 48]
	
	;; Moving argument number 3
	mov rdx, rax
	
	;; Saving non float register rdx's value
	push rdx
	
	;; Saving non float register rsi's value
	push rsi
	
	;; Saving non float register rdi's value
	push rdi
	
	lea rax, [rbp - 48]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	;; popping saved register value into rdi
	pop rdi
	;; popping saved register value into rsi
	pop rsi
	;; popping saved register value into rdx
	pop rdx
	
	;; Moving argument number 4
	mov rcx, rax
	
	call _string_ends_with
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rcx, 0
	mov rbx, rax
	cmp rbx, rcx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_16:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_16
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	mov rbx, [rbp - 80]
	
	mov rsi, rbx
	
	;; Saving non float register rax's value
	push rax
	
	;; Saving non float register rsi's value
	push rsi
	
	;; Saving non float register rdi's value
	push rdi
	
	lea rax, [rbp - 80]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	;; Moving function 'strlen' return value
	mov rbx, rax
	;; popping saved register value into rdi
	pop rdi
	;; popping saved register value into rsi
	pop rsi
	;; popping saved register value into rax
	pop rax
	
	mov rdx, rbx
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 248], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_12
	mov rdx, 37
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 248]
	
	call _print_int
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_13
	mov rdx, 23
	syscall
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	mov rbx, [rbp - 232]
	
	mov rsi, rbx
	
	mov rdx, [rbp - 240]
	
	syscall
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_14
	mov rdx, 1
	syscall
	
	mov rax, QWORD [CLOSE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	syscall
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_16
	.if_end_16:
	
	mov rax, [rbp - 176]
	
	;; Moving argument number 1
	mov rdi, rax
	
	;; Saving non float register rdi's value
	push rdi
	
	lea rax, [rbp - 176]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	;; popping saved register value into rdi
	pop rdi
	
	;; Moving argument number 2
	mov rsi, rax
	
	mov rax, [rbp - 232]
	
	;; Moving argument number 3
	mov rdx, rax
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 2966 ./examples/http_server/index.cy:111:26 Op(Minus) >
	mov rbx, [rbp - 208]
	mov rax, [rbp - 216]
	sub rax, rbx
	
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 2987 ./examples/http_server/index.cy:111:45 Op(Plus) >
	mov rbx, 1
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 1. second = rax. Locked: [rdi, rsi, rdx, rax, rbx, rcx]
	
	;; Moving argument number 4
	mov rcx, rbx
	
	;; Global new thingy
	xor rax, rax
	mov rax, file_to_read
	
	;; Moving argument number 5
	mov r8, rax
	
	call _str_concat
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 256], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_15
	mov rdx, 22
	syscall
	
	;; src/asm/internal_functions.rs:320
	mov rax, [rbp - 256]
	call _printRAX
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	mov rbx, [rbp - 232]
	
	mov rsi, rbx
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 3149 ./examples/http_server/index.cy:117:63 Op(Minus) >
	mov rcx, [rbp - 208]
	mov rbx, [rbp - 216]
	sub rbx, rcx
	
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 3170 ./examples/http_server/index.cy:117:82 Op(Plus) >
	mov rcx, 1
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = 1. second = rbx. Locked: [rax, rdi, rsi, rbx, rcx, rdx]
	
	mov rdx, rcx
	
	syscall
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_16
	mov rdx, 1
	syscall
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, file_to_read
	
	mov rsi, rbx
	
	mov rdx, [rbp - 256]
	
	syscall
	
	;; Global new thingy
	xor rax, rax
	mov rax, read_data
	
	;; Moving argument number 1
	mov rdi, rax
	
	
	;; Moving argument number 2
	mov rsi, 4096
	
	;; Global new thingy
	xor rax, rax
	mov rax, file_to_read
	
	;; Moving argument number 3
	mov rdx, rax
	
	call _read_file_into_memory
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 264], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 264]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_17:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_17
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	mov rbx, [rbp - 96]
	
	mov rsi, rbx
	
	;; Saving non float register rax's value
	push rax
	
	;; Saving non float register rsi's value
	push rsi
	
	;; Saving non float register rdi's value
	push rdi
	
	lea rax, [rbp - 96]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	;; Moving function 'strlen' return value
	mov rbx, rax
	;; popping saved register value into rdi
	pop rdi
	;; popping saved register value into rsi
	pop rsi
	;; popping saved register value into rax
	pop rax
	
	mov rdx, rbx
	
	syscall
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_17
	mov rdx, 32
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 264]
	
	call _print_int
	
	jmp .else_end_17
	.if_end_17:
	
	.else_17:
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_18
	mov rdx, 5
	syscall
	
	;; src/asm/internal_functions.rs:320
	mov rax, [rbp - 264]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_19
	mov rdx, 17
	syscall
	
	;; Global new thingy
	xor rax, rax
	mov rax, file_to_read
	
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_20
	mov rdx, 1
	syscall
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	mov rbx, [rbp - 112]
	
	mov rsi, rbx
	
	mov rdx, [rbp - 120]
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 272], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_21
	mov rdx, 28
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 272]
	
	call _print_int
	
	;; Global new thingy
	xor rax, rax
	mov rax, file_len
	
	;; Moving argument number 1
	mov rdi, rax
	
	;; Moving argument number 2
	mov rsi, [rbp - 264]
	
	call _write_int_into_mem
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 280], rax
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, file_len
	
	mov rsi, rbx
	
	mov rdx, [rbp - 280]
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 272], rbx
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	mov rbx, [rbp - 144]
	
	mov rsi, rbx
	
	mov rdx, [rbp - 152]
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 272], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_22
	mov rdx, 50
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 272]
	
	call _print_int
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, read_data
	
	mov rsi, rbx
	
	mov rdx, [rbp - 264]
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 272], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_23
	mov rdx, 28
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 272]
	
	call _print_int
	
	.else_end_17:
	mov rax, QWORD [CLOSE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	syscall
	
	;; 'http_index_html_len' at '[rbp - 120]'
	;; 'path_as_char' at '[rbp - 232]'
	;; 'num_written' at '[rbp - 280]'
	;; 'header_body_seperator' at '[rbp - 144]'
	;; 'path_ends_at_idx' at '[rbp - 216]'
	;; 'http_404' at '[rbp - 80]'
	;; 'http_ok' at '[rbp - 64]'
	;; 'index_html_file_dir_path' at '[rbp - 176]'
	;; 'file_read_bytes' at '[rbp - 264]'
	;; 'dot_html' at '[rbp - 48]'
	;; 'http_index_html' at '[rbp - 112]'
	;; 'idx' at '[rbp - 184]'
	;; 'read_bytes' at '[rbp - 24]'
	;; 'req' at '[rbp - 16]'
	;; 'character1' at '[rbp - 224]'
	;; 'write_ret' at '[rbp - 272]'
	;; 'connfd' at '[rbp - 8]'
	;; 'http_500' at '[rbp - 96]'
	;; 'header_body_seperator_len' at '[rbp - 152]'
	;; 'method_ends_at_idx' at '[rbp - 192]'
	;; 'path_starts_at_idx' at '[rbp - 208]'
	;; 'path_len' at '[rbp - 240]'
	;; 'final_file_abs_path' at '[rbp - 256]'
	;; 'character' at '[rbp - 200]'
	mov rsp, rbp
	pop rbp
	ret
	

_main:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 96
	
	mov rax, QWORD [SOCKET_SYSCALL]
	
	mov rdi, QWORD [AF_INET]
	
	mov rsi, QWORD [SOCK_STREAM]
	
	
	mov rdx, 0
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 8], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_24
	mov rdx, 23
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 8]
	
	call _print_int
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 8]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_18:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_18
	
	
	pop rdi
	mov rax, 60
	syscall
	
	jmp .if_end_18
	.if_end_18:
	
	;; Global new thingy
	xor rax, rax
	mov rax, serveraddr_mem
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'sa_prefix' of type Integer. times_dereferenced: 0
	mov rbx, rax
	mov [rbp - 16], rax
	
	;; Global new thingy
	xor rax, rax
	mov rax, serveraddr_mem
	
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 4671 ./examples/http_server/index.cy:160:40 Op(Plus) >
	mov rbx, 2
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 2. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'sin_port' of type Integer16. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 24], rax
	
	;; Global new thingy
	xor rax, rax
	mov rax, serveraddr_mem
	
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 4716 ./examples/http_server/index.cy:161:38 Op(Plus) >
	mov rbx, 4
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 4. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 's_addr' of type Integer32. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 32], rax
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'sa_prefix' of type Integer. times_dereferenced: 1
	mov rax, QWORD [AF_INET]
	mov rbx, [rbp - 16]
	mov [rbx], rax
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'sin_port' of type Integer16. times_dereferenced: 1
	mov ax, WORD [PORT]
	mov rbx, [rbp - 24]
	mov [rbx], ax
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 's_addr' of type Integer32. times_dereferenced: 1
	mov eax, DWORD [S_ADDR]
	mov rbx, [rbp - 32]
	mov [rbx], eax
	
	mov rax, QWORD [BIND_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, serveraddr_mem
	
	mov rsi, rbx
	
	
	mov rdx, 16
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_25
	mov rdx, 21
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 40]
	
	call _print_int
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 40]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_19:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_19
	
	
	pop rdi
	mov rax, 60
	syscall
	
	jmp .if_end_19
	.if_end_19:
	
	mov rax, QWORD [LISTEN_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	
	mov rsi, 10
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_26
	mov rdx, 23
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 48]
	
	call _print_int
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 48]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_20:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_20
	
	
	pop rdi
	mov rax, 60
	syscall
	
	jmp .if_end_20
	.if_end_20:
	
	.loop_11:
	mov rax, QWORD [ACCEPT_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	
	mov rsi, 0
	
	
	mov rdx, 0
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 56], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_27
	mov rdx, 23
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 56]
	
	call _print_int
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 56]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_21:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_21
	
	
	pop rdi
	mov rax, 60
	syscall
	
	jmp .if_end_21
	.if_end_21:
	
	mov rax, QWORD [READ_SYSCALL]
	
	mov rdi, [rbp - 56]
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, read_data
	
	mov rsi, rbx
	
	
	mov rdx, 4096
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 64], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_28
	mov rdx, 5
	syscall
	
	;; src/asm/internal_functions.rs:320
	mov rax, [rbp - 64]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_29
	mov rdx, 8
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 56]
	
	;; Global new thingy
	xor rax, rax
	mov rax, read_data
	
	;; Moving argument number 2
	mov rsi, rax
	
	;; Moving argument number 3
	mov rdx, [rbp - 64]
	
	call _parse_http_request
	
	.loop_11_end_start:
	jmp .loop_11
	.loop_end_11:
	
	;; 'read_bytes' at '[rbp - 64]'
	;; 'sa_prefix' at '[rbp - 16]'
	;; 'sin_port' at '[rbp - 24]'
	;; 'listener' at '[rbp - 48]'
	;; 's_addr' at '[rbp - 32]'
	;; 'connfd' at '[rbp - 56]'
	;; 'sockfd' at '[rbp - 8]'
	;; 'bind_ret' at '[rbp - 40]'
	mov rsp, rbp
	pop rbp
	ret
	

