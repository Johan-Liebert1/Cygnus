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
	buffer resb 1048576

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
	string_0 db 45 ;; < 583 ./examples/http_server/../include/std.cy:27:15 StringLiteral("-") >
	string_1 db 114,101,97,100,95,102,105,108,101,95,105,110,116,111,95,109,101,109,111,114,121,32,111,112,101,110,32,115,121,115,99,97,108,108,32,102,97,105,108,101,100,32,102,111,114,32,102,105,108,101,58,32 ;; < 1424 ./examples/http_server/../include/std.cy:71:66 StringLiteral("read_file_into_memory open syscall failed for file: ") >
	string_2 db 87,114,105,116,105,110,103,32,114,101,113,32,116,111,32,115,116,100,111,117,116,46,46,46,10 ;; < 662 ./examples/http_server/index.cy:35:40 StringLiteral("Writing req to stdout...\\n") >
	string_3 db 46,104,116,109,108 ;; < 759 ./examples/http_server/index.cy:39:28 StringLiteral(".html") >
	string_4 db 72,84,84,80,47,49,46,49,32,50,48,48,32,79,75,13,10,67,111,110,116,101,110,116,45,84,121,112,101,58,32,116,101,120,116,47,112,108,97,105,110,13,10,67,111,110,116,101,110,116,45,76,101,110,103,116,104,58,32,50,13,10,13,10,79,75 ;; < 861 ./examples/http_server/index.cy:41:96 StringLiteral("HTTP/1.1 200 OK\\r\\nContent-Type: text/plain\\r\\nContent-Length: 2\\r\\n\\r\\nOK") >
	string_5 db 72,84,84,80,47,49,46,49,32,52,48,52,32,78,79,84,32,70,79,85,78,68,13,10,13,10 ;; < 919 ./examples/http_server/index.cy:42:53 StringLiteral("HTTP/1.1 404 NOT FOUND\\r\\n\\r\\n") >
	string_6 db 72,84,84,80,47,49,46,49,32,53,48,48,32,73,110,116,101,114,110,97,108,32,83,101,114,118,101,114,32,69,114,114,111,114,13,10,13,10 ;; < 989 ./examples/http_server/index.cy:43:65 StringLiteral("HTTP/1.1 500 Internal Server Error\\r\\n\\r\\n") >
	string_7 db 72,84,84,80,47,49,46,49,32,50,48,48,32,79,75,13,10,67,111,110,116,101,110,116,45,84,121,112,101,58,32,116,101,120,116,47,104,116,109,108,13,10,67,111,110,116,101,110,116,45,76,101,110,103,116,104,58,32 ;; < 1095 ./examples/http_server/index.cy:45:100 StringLiteral("HTTP/1.1 200 OK\\r\\nContent-Type: text/html\\r\\nContent-Length: ") >
	string_8 db 104,116,116,112,95,104,101,97,100,101,114,95,102,111,114,95,99,111,110,116,101,110,116,95,108,101,110,32,61,32 ;; < 1217 ./examples/http_server/index.cy:48:40 StringLiteral("http_header_for_content_len = ") >
	string_9 db 13,10,13,10 ;; < 1296 ./examples/http_server/index.cy:50:44 StringLiteral("\\r\\n\\r\\n") >
	string_10 db 47,104,111,109,101,47,112,114,97,103,121,97,110,47,82,117,115,116,47,108,97,110,103,47,101,120,97,109,112,108,101,115,47,104,116,116,112,95,115,101,114,118,101,114 ;; < 1458 ./examples/http_server/index.cy:53:83 StringLiteral("/home/pragyan/Rust/lang/examples/http_server") >
	string_11 db 109,101,116,104,111,100,95,101,110,100,115,95,97,116,95,105,100,120,32,61,32 ;; < 1848 ./examples/http_server/index.cy:72:31 StringLiteral("method_ends_at_idx = ") >
	string_12 db 112,97,116,104,95,97,115,95,99,104,97,114,32,61,32 ;; < 2384 ./examples/http_server/index.cy:95:25 StringLiteral("path_as_char = ") >
	string_13 db 10 ;; < 2459 ./examples/http_server/index.cy:97:12 StringLiteral("\\n") >
	string_14 db 87,114,105,116,105,110,103,32,104,116,116,112,95,52,48,52,32,116,111,32,99,111,110,110,102,100,32,114,101,116,117,114,110,101,100,58,32 ;; < 2708 ./examples/http_server/index.cy:102:51 StringLiteral("Writing http_404 to connfd returned: ") >
	string_15 db 67,108,105,101,110,116,32,97,115,107,101,100,32,102,111,114,32,112,97,116,104,58,32 ;; < 2779 ./examples/http_server/index.cy:104:37 StringLiteral("Client asked for path: ") >
	string_16 db 10 ;; < 2862 ./examples/http_server/index.cy:106:16 StringLiteral("\\n") >
	string_17 db 102,105,110,97,108,95,102,105,108,101,95,97,98,115,95,112,97,116,104,95,108,101,110,32,61,32 ;; < 3402 ./examples/http_server/index.cy:126:36 StringLiteral("final_file_abs_path_len = ") >
	string_18 db 10 ;; < 3540 ./examples/http_server/index.cy:129:12 StringLiteral("\\n") >
	string_19 db 114,101,97,100,95,102,105,108,101,95,105,110,116,111,95,109,101,109,111,114,121,32,114,101,116,117,114,110,101,100,58,32 ;; < 3867 ./examples/http_server/index.cy:136:46 StringLiteral("read_file_into_memory returned: ") >
	string_20 db 82,101,97,100,32 ;; < 3938 ./examples/http_server/index.cy:139:19 StringLiteral("Read ") >
	string_21 db 32,98,121,116,101,115,32,102,114,111,109,32,102,105,108,101,32,10 ;; < 3978 ./examples/http_server/index.cy:139:55 StringLiteral(" bytes from file \\n") >
	string_22 db 87,114,111,116,101,32 ;; < 4706 ./examples/http_server/index.cy:155:20 StringLiteral("Wrote ") >
	string_23 db 32,105,110,116,111,32,99,111,110,110,102,100,10 ;; < 4735 ./examples/http_server/index.cy:155:45 StringLiteral(" into connfd\\n") >
	string_24 db 83,79,67,75,69,84,95,83,89,83,67,65,76,76,32,114,101,116,117,114,110,58,32 ;; < 4965 ./examples/http_server/index.cy:165:33 StringLiteral("SOCKET_SYSCALL return: ") >
	string_25 db 66,73,78,68,95,83,89,83,67,65,76,76,32,114,101,116,117,114,110,58,32 ;; < 5350 ./examples/http_server/index.cy:181:31 StringLiteral("BIND_SYSCALL return: ") >
	string_26 db 76,73,83,84,69,78,95,83,89,83,67,65,76,76,32,114,101,116,117,114,110,58,32 ;; < 5521 ./examples/http_server/index.cy:189:33 StringLiteral("LISTEN_SYSCALL return: ") >
	string_27 db 65,67,67,69,80,84,95,83,89,83,67,65,76,76,32,114,101,116,117,114,110,58,32 ;; < 5710 ./examples/http_server/index.cy:197:37 StringLiteral("ACCEPT_SYSCALL return: ") >
	string_28 db 82,101,97,100,32 ;; < 5905 ./examples/http_server/index.cy:206:19 StringLiteral("Read ") >
	string_29 db 32,98,121,116,101,115,32,10 ;; < 5930 ./examples/http_server/index.cy:206:40 StringLiteral(" bytes \\n") >

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
	mov rax, [rbp - 8]
	mov rbx, 1
	sub rax, rbx
	;; will lock rax. first = 1. second = [rbp - 8]. Locked: [rax, rbx]
	
	not rbx
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 16], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_0
	mov rdx, 1
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 16], type: Integer
	;; no xor here
	mov rax, [rbp - 16]
	call _printRAX
	
	jmp .else_end_0
	.if_end_0:
	
	.else_0:
	;; assign_local_number of type Integer
	mov rax, [rbp - 8]
	mov QWORD [rbp - 16], rax
	
	;; get_vec_for_write_number. stack_member: [rbp - 16], type: Integer
	;; no xor here
	mov rax, [rbp - 16]
	call _printRAX
	
	.else_end_0:
	;; 'n' at '[rbp - 16]'
	;; 'a' at '[rbp - 8]'
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
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'thing' of type Integer. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 64], rax
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'thing' of type Integer. times_dereferenced: 1
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
	
	;; 'loop_0_from' at '[rbp - 40]'
	;; 'value' at '[rbp - 16]'
	;; 'loop_0_to' at '[rbp - 48]'
	;; 'i' at '[rbp - 32]'
	;; 'loop_0_step' at '[rbp - 56]'
	;; 'size' at '[rbp - 24]'
	;; 'ptr' at '[rbp - 8]'
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
	mov bl, [rax]
	
	
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
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'len' of type String. times_dereferenced: 0
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
	
	;; 'string' at '[rbp - 8]'
	;; 'len' at '[rbp - 16]'
	;; 'length' at '[rbp - 24]'

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
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 52
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 32]
	
	call _print_int
	
	
	;; Returning from function
	mov rax, -1
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_2
	.if_end_2:
	
	mov rax, QWORD [READ_SYSCALL]
	
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
	;; 'memory' at '[rbp - 8]'
	;; 'abs_file_path' at '[rbp - 24]'
	;; 'mem_size' at '[rbp - 16]'
	;; 'read_bytes' at '[rbp - 40]'

_write_int_into_mem:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 112
	;; param name memory. Param type Pointer -> Integer
	mov [rbp - 8], rdi
	;; param name number. Param type Integer
	mov [rbp - 16], rsi
	
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 17], 48
	
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
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 1968 ./examples/http_server/../include/std.cy:100:29 Op(Minus) >
	mov rax, [rbp - 40]
	mov rbx, 1
	sub rax, rbx
	;; will lock rax. first = 1. second = [rbp - 40]. Locked: [rax, rbx]
	
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
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 49], dl
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 2072 ./examples/http_server/../include/std.cy:106:39 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'idx_into_mem' of type Integer8. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 64], rax
	
	;; Plus get the two operands from the stack. Result type: Integer8. Token: < 2105 ./examples/http_server/../include/std.cy:107:25 Op(Plus) >
	mov rax, 0 ;; clearing al
	mov rbx, 0 ;; clearing bl
	mov al, [rbp - 17]
	mov bl, [rbp - 49]
	add al, bl
	;; will lock rax. first = [rbp - 17]. second = [rbp - 49]. Locked: [rax, rbx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'idx_into_mem' of type Integer8. times_dereferenced: 1
	mov bl, al
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
	
	;; 'idx_into_mem' at '[rbp - 64]'
	;; 'number' at '[rbp - 16]'
	;; 'number_len' at '[rbp - 40]'
	;; 'memory' at '[rbp - 8]'
	;; 'idx' at '[rbp - 48]'
	;; 'n' at '[rbp - 32]'
	;; 'c' at '[rbp - 49]'
	;; 'zero_ascii' at '[rbp - 17]'

_write_str_into_mem:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 64
	;; param name memory. Param type Pointer -> Character
	mov [rbp - 8], rdi
	;; param name string. Param type Pointer -> Character
	mov [rbp - 16], rsi
	;; param name len. Param type Integer
	mov [rbp - 24], rdx
	
	
	
	;; loop_4 start
	mov QWORD [rbp - 48], 1 ;; step
	mov rax, [rbp - 24] ;; to
	mov QWORD [rbp - 40], rax ;; to
	mov QWORD [rbp - 32], 0 ;; from
	mov QWORD [rbp - 56], 0 ;; loop variable i
	.loop_4:
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 2376 ./examples/http_server/../include/std.cy:122:31 Op(Plus) >
	mov rbx, [rbp - 56]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 56]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'memo' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 64], rax
	
	mov rax, [rbp - 16]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 2406 ./examples/http_server/../include/std.cy:123:22 Op(Plus) >
	mov rbx, [rbp - 56]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 56]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	mov rax, rbx
	xor rbx, rbx
	mov bl, [rax]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'memo' of type Character. times_dereferenced: 1
	mov al, bl
	mov rcx, [rbp - 64]
	mov [rcx], al
	
	.loop_4_end_start:
	;; inc the loop variable
	mov rax, [rbp - 56]
	mov rbx, [rbp - 48]
	add rax, rbx
	mov [rbp - 56], rax
	;; check exit condition
	mov rcx, [rbp - 48] ;; step
	mov rbx, [rbp - 40] ;; to
	mov rax, [rbp - 32] ;; from
	add rax, rcx
	dec rbx
	cmp rax, rbx
	jg .loop_end_4
	mov [rbp - 32], rax
	jmp .loop_4
	.loop_end_4:
	
	;; Returning from function
	mov rax, [rbp - 24]
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'loop_4_to' at '[rbp - 40]'
	;; 'loop_4_from' at '[rbp - 32]'
	;; 'loop_4_step' at '[rbp - 48]'
	;; 'memory' at '[rbp - 8]'
	;; 'string' at '[rbp - 16]'
	;; 'len' at '[rbp - 24]'

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
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 2734 ./examples/http_server/../include/std.cy:135:38 Op(Minus) >
	mov rax, [rbp - 16]
	mov rbx, 1
	sub rax, rbx
	;; will lock rax. first = 1. second = [rbp - 16]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 40], rax
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 2781 ./examples/http_server/../include/std.cy:136:41 Op(Minus) >
	mov rax, [rbp - 32]
	mov rbx, 1
	sub rax, rbx
	;; will lock rax. first = 1. second = [rbp - 32]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], rax
	
	.loop_5:
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 2835 ./examples/http_server/../include/std.cy:139:35 Op(Plus) >
	mov rbx, [rbp - 40]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 40]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'str_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 2891 ./examples/http_server/../include/std.cy:140:38 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'substr_char' of type Character. times_dereferenced: 0
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
	jmp .loop_end_5
	
	jmp .if_end_7
	.if_end_7:
	
	.loop_5_end_start:
	jmp .loop_5
	.loop_end_5:
	
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 3134 ./examples/http_server/../include/std.cy:154:25 Op(Plus) >
	mov rax, 1
	mov rbx, [rbp - 40]
	add rax, rbx
	;; will lock rax. first = 1. second = [rbp - 40]. Locked: [rax, rbx]
	
	;; Returning from function
	mov rax, rax
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'string' at '[rbp - 8]'
	;; 'idx_into_str' at '[rbp - 40]'
	;; 'substr_char' at '[rbp - 64]'
	;; 'substr_len' at '[rbp - 32]'
	;; 'idx_into_substr' at '[rbp - 48]'
	;; 'substr' at '[rbp - 24]'
	;; 'string_len' at '[rbp - 16]'
	;; 'str_char' at '[rbp - 56]'

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
	
	.loop_6:
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 3456 ./examples/http_server/../include/std.cy:167:35 Op(Plus) >
	mov rbx, [rbp - 40]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 40]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'str_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 3512 ./examples/http_server/../include/std.cy:168:38 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'substr_char' of type Character. times_dereferenced: 0
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
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 3703 ./examples/http_server/../include/std.cy:177:40 Op(Minus) >
	mov rax, [rbp - 32]
	mov rbx, 1
	sub rax, rbx
	;; will lock rax. first = 1. second = [rbp - 32]. Locked: [rax, rbx]
	
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
	jmp .loop_end_6
	
	jmp .if_end_10
	.if_end_10:
	
	.loop_6_end_start:
	jmp .loop_6
	.loop_end_6:
	
	
	;; Returning from function
	mov rax, 1
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'substr' at '[rbp - 24]'
	;; 'idx_into_substr' at '[rbp - 48]'
	;; 'string' at '[rbp - 8]'
	;; 'substr_len' at '[rbp - 32]'
	;; 'substr_char' at '[rbp - 64]'
	;; 'string_len' at '[rbp - 16]'
	;; 'idx_into_str' at '[rbp - 40]'
	;; 'str_char' at '[rbp - 56]'

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
	
	.loop_7:
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4025 ./examples/http_server/../include/std.cy:194:35 Op(Plus) >
	mov rbx, [rbp - 40]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 40]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'str_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 56], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4082 ./examples/http_server/../include/std.cy:195:39 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'substr_char' of type Character. times_dereferenced: 0
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
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 4273 ./examples/http_server/../include/std.cy:204:40 Op(Minus) >
	mov rax, [rbp - 16]
	mov rbx, 1
	sub rax, rbx
	;; will lock rax. first = 1. second = [rbp - 16]. Locked: [rax, rbx]
	
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
	jmp .loop_end_7
	
	jmp .if_end_13
	.if_end_13:
	
	.loop_7_end_start:
	jmp .loop_7
	.loop_end_7:
	
	
	;; Returning from function
	mov rax, 1
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'string2_len' at '[rbp - 32]'
	;; 'idx_into_str' at '[rbp - 40]'
	;; 'str_char' at '[rbp - 56]'
	;; 'string2' at '[rbp - 24]'
	;; 'string' at '[rbp - 8]'
	;; 'idx_into_substr' at '[rbp - 48]'
	;; 'string_len' at '[rbp - 16]'
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
	
	
	
	;; loop_8 start
	mov QWORD [rbp - 72], 1 ;; step
	mov rax, [rbp - 16] ;; to
	mov QWORD [rbp - 64], rax ;; to
	mov QWORD [rbp - 56], 0 ;; from
	mov QWORD [rbp - 80], 0 ;; loop variable i
	.loop_8:
	
	mov rax, [rbp - 40]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4554 ./examples/http_server/../include/std.cy:216:39 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'new_str_idx' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 88], rax
	
	mov rax, [rbp - 8]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4611 ./examples/http_server/../include/std.cy:217:35 Op(Plus) >
	mov rbx, [rbp - 80]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 80]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'old_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 96], rax
	
	mov rbx, [rbp - 96]
	xor rax, rax
	mov al, [rbx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'new_str_idx' of type Character. times_dereferenced: 1
	mov bl, al
	mov rcx, [rbp - 88]
	mov [rcx], bl
	
	
	mov rax, [rbp - 48]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 48], rax
	
	.loop_8_end_start:
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
	jg .loop_end_8
	mov [rbp - 56], rax
	jmp .loop_8
	.loop_end_8:
	
	
	
	;; loop_9 start
	mov QWORD [rbp - 120], 1 ;; step
	mov rax, [rbp - 32] ;; to
	mov QWORD [rbp - 112], rax ;; to
	mov QWORD [rbp - 104], 0 ;; from
	mov QWORD [rbp - 128], 0 ;; loop variable i
	.loop_9:
	
	mov rax, [rbp - 40]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4771 ./examples/http_server/../include/std.cy:225:39 Op(Plus) >
	mov rbx, [rbp - 48]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 48]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'new_str_idx' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 136], rax
	
	mov rax, [rbp - 24]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Character. Token: < 4829 ./examples/http_server/../include/std.cy:226:36 Op(Plus) >
	mov rbx, [rbp - 128]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 128]. second = rax. Locked: [rax, rbx, rcx]
	;; binary op ptr -> char
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'old_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 144], rax
	
	mov rbx, [rbp - 144]
	xor rax, rax
	mov al, [rbx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'new_str_idx' of type Character. times_dereferenced: 1
	mov bl, al
	mov rcx, [rbp - 136]
	mov [rcx], bl
	
	
	mov rax, [rbp - 48]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 48], rax
	
	.loop_9_end_start:
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
	jg .loop_end_9
	mov [rbp - 104], rax
	jmp .loop_9
	.loop_end_9:
	
	;; Returning from function
	mov rax, [rbp - 48]
	
	mov rsp, rbp
	pop rbp
	ret
	
	;; 'loop_9_to' at '[rbp - 112]'
	;; 'loop_9_step' at '[rbp - 120]'
	;; 'idx_into_new_str' at '[rbp - 48]'
	;; 'string' at '[rbp - 8]'
	;; 'new_str' at '[rbp - 40]'
	;; 'loop_8_to' at '[rbp - 64]'
	;; 'string2_len' at '[rbp - 32]'
	;; 'string2' at '[rbp - 24]'
	;; 'loop_8_step' at '[rbp - 72]'
	;; 'loop_9_from' at '[rbp - 104]'
	;; 'loop_8_from' at '[rbp - 56]'
	;; 'string_len' at '[rbp - 16]'

_parse_http_request:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 352
	;; param name connfd. Param type Integer
	mov [rbp - 8], rdi
	;; param name req. Param type Pointer -> Integer8
	mov [rbp - 16], rsi
	;; param name read_bytes. Param type Integer
	mov [rbp - 24], rdx
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], 1
	
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 33], 32
	
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 34], 10
	
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 35], 0
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 1
	mov rax, [rbp - 32]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_14:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_14
	
	
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
	
	jmp .if_end_14
	.if_end_14:
	
	
	mov QWORD [rbp - 56], 5
	mov QWORD [rbp - 64], string_3
	
	
	mov QWORD [rbp - 72], 66
	mov QWORD [rbp - 80], string_4
	
	
	mov QWORD [rbp - 88], 26
	mov QWORD [rbp - 96], string_5
	
	
	mov QWORD [rbp - 104], 38
	mov QWORD [rbp - 112], string_6
	
	
	mov QWORD [rbp - 120], 58
	mov QWORD [rbp - 128], string_7
	
	lea rax, [rbp - 128]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 136], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_8
	mov rdx, 30
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 136], type: Integer
	;; no xor here
	mov rax, [rbp - 136]
	call _printRAX
	
	
	mov QWORD [rbp - 152], 4
	mov QWORD [rbp - 160], string_9
	
	lea rax, [rbp - 160]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 168], rax
	
	
	mov QWORD [rbp - 184], 44
	mov QWORD [rbp - 192], string_10
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 200], 0
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 208], 0
	
	.loop_10:
	mov rax, [rbp - 16]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer8. Token: < 1646 ./examples/http_server/index.cy:62:33 Op(Plus) >
	mov rbx, [rbp - 200]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 200]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'character' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 216], rax
	
	mov rbx, [rbp - 216]
	xor rax, rax
	mov al, [rbx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov cl, [rbp - 33]
	mov bl, al
	cmp bl, cl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	mov rcx, [rbp - 216]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov dl, [rbp - 35]
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
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 1757 ./examples/http_server/index.cy:65:37 Op(Minus) >
	mov rax, [rbp - 200]
	mov rbx, 1
	sub rax, rbx
	;; will lock rax. first = 1. second = [rbp - 200]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 208], rax
	
	;; --- break ----
	jmp .loop_end_10
	
	jmp .if_end_15
	.if_end_15:
	
	
	mov rax, [rbp - 200]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 200], rax
	
	.loop_10_end_start:
	jmp .loop_10
	.loop_end_10:
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_11
	mov rdx, 21
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 208], type: Integer
	;; no xor here
	mov rax, [rbp - 208]
	call _printRAX
	
	
	mov rax, [rbp - 200]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 200], rax
	
	;; assign_local_number of type Integer
	mov rax, [rbp - 200]
	mov QWORD [rbp - 224], rax
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 232], -1
	
	.loop_11:
	mov rax, [rbp - 16]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer8. Token: < 2064 ./examples/http_server/index.cy:82:34 Op(Plus) >
	mov rbx, [rbp - 200]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 200]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'character1' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 240], rax
	
	mov rbx, [rbp - 240]
	xor rax, rax
	mov al, [rbx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov cl, [rbp - 33]
	mov bl, al
	cmp bl, cl
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	mov rcx, [rbp - 240]
	xor rbx, rbx
	mov bl, [rcx]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov dl, [rbp - 35]
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
	
	.if_16:
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_16
	
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 2175 ./examples/http_server/index.cy:85:35 Op(Minus) >
	mov rax, [rbp - 200]
	mov rbx, 1
	sub rax, rbx
	;; will lock rax. first = 1. second = [rbp - 200]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 232], rax
	
	;; --- break ----
	jmp .loop_end_11
	
	jmp .if_end_16
	.if_end_16:
	
	
	mov rax, [rbp - 200]
	mov rbx, 1
	add rax, rbx
	mov [rbp - 200], rax
	
	.loop_11_end_start:
	jmp .loop_11
	.loop_end_11:
	
	mov rax, [rbp - 16]
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer8. Token: < 2269 ./examples/http_server/index.cy:92:32 Op(Plus) >
	mov rbx, [rbp - 224]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 224]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'path_as_char' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 248], rax
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 2332 ./examples/http_server/index.cy:93:40 Op(Minus) >
	mov rax, [rbp - 232]
	mov rbx, [rbp - 224]
	sub rax, rbx
	;; will lock rax. first = [rbp - 224]. second = [rbp - 232]. Locked: [rax, rbx]
	
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 2353 ./examples/http_server/index.cy:93:59 Op(Plus) >
	mov rbx, 1
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 1. second = rax. Locked: [rax, rbx, rcx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 256], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_12
	mov rdx, 15
	syscall
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	mov rbx, [rbp - 248]
	
	mov rsi, rbx
	
	mov rdx, [rbp - 256]
	
	syscall
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_13
	mov rdx, 1
	syscall
	
	mov rax, [rbp - 248]
	
	;; Moving argument number 1
	mov rdi, rax
	
	;; Moving argument number 2
	mov rsi, [rbp - 256]
	
	mov rax, [rbp - 64]
	
	;; Moving argument number 3
	mov rdx, rax
	
	;; Saving non float register rdx's value
	push rdx
	
	;; Saving non float register rsi's value
	push rsi
	
	;; Saving non float register rdi's value
	push rdi
	
	lea rax, [rbp - 64]
	
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
	
	.if_17:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_17
	
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
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 264], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_14
	mov rdx, 37
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 264]
	
	call _print_int
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_15
	mov rdx, 23
	syscall
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	mov rbx, [rbp - 248]
	
	mov rsi, rbx
	
	mov rdx, [rbp - 256]
	
	syscall
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_16
	mov rdx, 1
	syscall
	
	mov rax, QWORD [CLOSE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	syscall
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_17
	.if_end_17:
	
	mov rax, [rbp - 192]
	
	;; Moving argument number 1
	mov rdi, rax
	
	;; Saving non float register rdi's value
	push rdi
	
	lea rax, [rbp - 192]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call _strlen
	;; popping saved register value into rdi
	pop rdi
	
	;; Moving argument number 2
	mov rsi, rax
	
	mov rax, [rbp - 248]
	
	;; Moving argument number 3
	mov rdx, rax
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 3118 ./examples/http_server/index.cy:116:26 Op(Minus) >
	mov rax, [rbp - 232]
	mov rbx, [rbp - 224]
	sub rax, rbx
	;; will lock rax. first = [rbp - 224]. second = [rbp - 232]. Locked: [rdi, rsi, rdx, rax, rbx]
	
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 3139 ./examples/http_server/index.cy:116:45 Op(Plus) >
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
	mov QWORD [rbp - 272], rax
	
	;; Global new thingy
	xor rax, rax
	mov rax, file_to_read
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 3318 ./examples/http_server/index.cy:123:37 Op(Plus) >
	mov rbx, [rbp - 272]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 272]. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'to_write' of type Character. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 280], rax
	
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'to_write' of type Character. times_dereferenced: 1
	mov al, 0
	mov rbx, [rbp - 280]
	mov [rbx], al
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_17
	mov rdx, 26
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 272], type: Integer
	;; no xor here
	mov rax, [rbp - 272]
	call _printRAX
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	mov rbx, [rbp - 248]
	
	mov rsi, rbx
	
	;; Minus get the two operands from the stack. Result type: Integer. Token: < 3502 ./examples/http_server/index.cy:128:63 Op(Minus) >
	mov rbx, [rbp - 232]
	mov rcx, [rbp - 224]
	sub rbx, rcx
	;; will lock rbx. first = [rbp - 224]. second = [rbp - 232]. Locked: [rax, rdi, rsi, rbx, rcx]
	
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 3523 ./examples/http_server/index.cy:128:82 Op(Plus) >
	mov rcx, 1
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = 1. second = rbx. Locked: [rax, rdi, rsi, rbx, rcx, rdx]
	
	mov rdx, rcx
	
	syscall
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_18
	mov rdx, 1
	syscall
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, file_to_read
	
	mov rsi, rbx
	
	mov rdx, [rbp - 272]
	
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
	mov QWORD [rbp - 288], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 288]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags. Also cmove instruction set only takes reg, reg/mem as arguments
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_18:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_18
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	mov rbx, [rbp - 112]
	
	mov rsi, rbx
	
	;; Saving non float register rax's value
	push rax
	
	;; Saving non float register rsi's value
	push rsi
	
	;; Saving non float register rdi's value
	push rdi
	
	lea rax, [rbp - 112]
	
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
	mov rsi, string_19
	mov rdx, 32
	syscall
	
	;; Moving argument number 1
	mov rdi, [rbp - 288]
	
	call _print_int
	
	jmp .else_end_18
	.if_end_18:
	
	.else_18:
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_20
	mov rdx, 5
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 288], type: Integer
	;; no xor here
	mov rax, [rbp - 288]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_21
	mov rdx, 18
	syscall
	
	;; Global new thingy
	xor rax, rax
	mov rax, buffer
	
	;; Moving argument number 1
	mov rdi, rax
	
	
	;; Moving argument number 2
	mov rsi, 0
	
	
	
	;; Multiply get the two operands from the stack. Type: Integer
	xor rdx, rdx
	mov rax, 1024
	mov rbx, 1024
	mul rbx
	
	;; Moving argument number 3
	mov rdx, rax
	
	call _memset
	
	;; Global new thingy
	xor rax, rax
	mov rax, buffer
	
	;; Moving argument number 1
	mov rdi, rax
	
	mov rax, [rbp - 128]
	
	;; Moving argument number 2
	mov rsi, rax
	
	;; Moving argument number 3
	mov rdx, [rbp - 136]
	
	call _write_str_into_mem
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 296], rax
	
	;; Global new thingy
	xor rax, rax
	mov rax, buffer
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 4269 ./examples/http_server/index.cy:147:47 Op(Plus) >
	mov rbx, [rbp - 296]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 296]. second = rax. Locked: [rax, rbx, rcx]
	
	;; Moving argument number 1
	mov rdi, rbx
	
	;; Moving argument number 2
	mov rsi, [rbp - 288]
	
	call _write_int_into_mem
	
	mov rbx, [rbp - 296]
	mov rcx, rax
	add rbx, rcx
	mov [rbp - 296], rbx
	
	;; Global new thingy
	xor rax, rax
	mov rax, buffer
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 4389 ./examples/http_server/index.cy:149:47 Op(Plus) >
	mov rbx, [rbp - 296]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 296]. second = rax. Locked: [rax, rbx, rcx]
	
	;; Moving argument number 1
	mov rdi, rbx
	
	mov rax, [rbp - 160]
	
	;; Moving argument number 2
	mov rsi, rax
	
	;; Moving argument number 3
	mov rdx, [rbp - 168]
	
	call _write_str_into_mem
	
	mov rbx, [rbp - 296]
	mov rcx, rax
	add rbx, rcx
	mov [rbp - 296], rbx
	
	;; Global new thingy
	xor rax, rax
	mov rax, buffer
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 4559 ./examples/http_server/index.cy:151:47 Op(Plus) >
	mov rbx, [rbp - 296]
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = [rbp - 296]. second = rax. Locked: [rax, rbx, rcx]
	
	;; Moving argument number 1
	mov rdi, rbx
	
	;; Global new thingy
	xor rax, rax
	mov rax, read_data
	
	;; Moving argument number 2
	mov rsi, rax
	
	;; Moving argument number 3
	mov rdx, [rbp - 288]
	
	call _write_str_into_mem
	
	mov rbx, [rbp - 296]
	mov rcx, rax
	add rbx, rcx
	mov [rbp - 296], rbx
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, buffer
	
	mov rsi, rbx
	
	mov rdx, [rbp - 296]
	
	syscall
	;; Moving function 'sycall' return value
	mov rbx, rax
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 304], rbx
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_22
	mov rdx, 6
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 304], type: Integer
	;; no xor here
	mov rax, [rbp - 304]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_23
	mov rdx, 13
	syscall
	
	mov rax, QWORD [WRITE_SYSCALL]
	
	mov rdi, QWORD [STDOUT]
	
	;; Global new thingy
	xor rbx, rbx
	mov rbx, buffer
	
	mov rsi, rbx
	
	mov rdx, [rbp - 296]
	
	syscall
	
	.else_end_18:
	mov rax, QWORD [CLOSE_SYSCALL]
	
	mov rdi, [rbp - 8]
	
	syscall
	
	;; 'http_ok' at '[rbp - 80]'
	;; 'method_ends_at_idx' at '[rbp - 208]'
	;; 'dot_html' at '[rbp - 64]'
	;; 'write_ret' at '[rbp - 304]'
	;; 'final_file_abs_path_len' at '[rbp - 272]'
	;; 'header_body_seperator' at '[rbp - 160]'
	;; 'to_write' at '[rbp - 280]'
	;; 'http_header_for_content_len' at '[rbp - 136]'
	;; 'connfd' at '[rbp - 8]'
	;; 'character1' at '[rbp - 240]'
	;; 'character' at '[rbp - 216]'
	;; 'path_len' at '[rbp - 256]'
	;; 'idx' at '[rbp - 200]'
	;; 'NEW_LINE_ASCII' at '[rbp - 34]'
	;; 'path_ends_at_idx' at '[rbp - 232]'
	;; 'file_read_bytes' at '[rbp - 288]'
	;; 'path_starts_at_idx' at '[rbp - 224]'
	;; 'path_as_char' at '[rbp - 248]'
	;; 'http_404' at '[rbp - 96]'
	;; 'read_bytes' at '[rbp - 24]'
	;; 'http_header_for_content' at '[rbp - 128]'
	;; 'SPACE_ASCII' at '[rbp - 33]'
	;; 'current_ptr' at '[rbp - 296]'
	;; 'req' at '[rbp - 16]'
	;; 'NULL_BYTE' at '[rbp - 35]'
	;; 'index_html_file_dir_path' at '[rbp - 192]'
	;; 'http_500' at '[rbp - 112]'
	;; 'PRINT_REQ' at '[rbp - 32]'
	;; 'header_body_seperator_len' at '[rbp - 168]'
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
	
	.if_19:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_19
	
	
	pop rdi
	mov rax, 60
	syscall
	
	jmp .if_end_19
	.if_end_19:
	
	;; Global new thingy
	xor rax, rax
	mov rax, serveraddr_mem
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'sa_prefix' of type Integer. times_dereferenced: 0
	mov rbx, rax
	mov [rbp - 16], rax
	
	;; Global new thingy
	xor rax, rax
	mov rax, serveraddr_mem
	
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 5122 ./examples/http_server/index.cy:173:40 Op(Plus) >
	mov rbx, 2
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 2. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'sin_port' of type Integer16. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 24], rax
	
	;; Global new thingy
	xor rax, rax
	mov rax, serveraddr_mem
	
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 5167 ./examples/http_server/index.cy:174:38 Op(Plus) >
	mov rbx, 4
	mov rcx, rax
	add rbx, rcx
	;; will lock rbx. first = 4. second = rax. Locked: [rax, rbx, rcx]
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 's_addr' of type Integer32. times_dereferenced: 0
	mov rax, rbx
	mov [rbp - 32], rax
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'sa_prefix' of type Integer. times_dereferenced: 1
	mov rax, QWORD [AF_INET]
	mov rbx, [rbp - 16]
	mov [rbx], rax
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 'sin_port' of type Integer16. times_dereferenced: 1
	mov ax, WORD [PORT]
	mov rbx, [rbp - 24]
	mov [rbx], ax
	
	;; src/asm/variable_assignment.rs:232 assign_local_pointer var 's_addr' of type Integer32. times_dereferenced: 1
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
	
	.if_20:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_20
	
	
	pop rdi
	mov rax, 60
	syscall
	
	jmp .if_end_20
	.if_end_20:
	
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
	
	.if_21:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_21
	
	
	pop rdi
	mov rax, 60
	syscall
	
	jmp .if_end_21
	.if_end_21:
	
	.loop_12:
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
	
	.if_22:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_22
	
	
	pop rdi
	mov rax, 60
	syscall
	
	jmp .if_end_22
	.if_end_22:
	
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
	
	;; get_vec_for_write_number. stack_member: [rbp - 64], type: Integer
	;; no xor here
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
	
	.loop_12_end_start:
	jmp .loop_12
	.loop_end_12:
	
	;; 'read_bytes' at '[rbp - 64]'
	;; 'sa_prefix' at '[rbp - 16]'
	;; 'bind_ret' at '[rbp - 40]'
	;; 's_addr' at '[rbp - 32]'
	;; 'sockfd' at '[rbp - 8]'
	;; 'connfd' at '[rbp - 56]'
	;; 'sin_port' at '[rbp - 24]'
	;; 'listener' at '[rbp - 48]'
	mov rsp, rbp
	pop rbp
	ret
	

