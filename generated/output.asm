%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 72,101,108,108,111,32,87,111,114,108,100,10 ;; < 202 ./examples/struct_assign.cy:10:62 StringLiteral("Hello World\\n") >
	string_1 db 120,32,61,32 ;; < 222 ./examples/struct_assign.cy:12:14 StringLiteral("x = ") >
	string_2 db 121,32,61,32 ;; < 246 ./examples/struct_assign.cy:13:14 StringLiteral("y = ") >
	string_3 db 122,32,61,32 ;; < 270 ./examples/struct_assign.cy:14:14 StringLiteral("z = ") >
	string_4 db 119,32,61,32 ;; < 294 ./examples/struct_assign.cy:15:14 StringLiteral("w = ") >
	string_5 db 104,101,108,108,111,32,61,32 ;; < 323 ./examples/struct_assign.cy:16:18 StringLiteral("hello = ") >
	string_6 db 71,111,111,100,98,121,101,32,87,111,114,108,100,10 ;; < 427 ./examples/struct_assign.cy:21:28 StringLiteral("Goodbye World\\n") >
	string_7 db 120,32,61,32 ;; < 445 ./examples/struct_assign.cy:23:14 StringLiteral("x = ") >
	string_8 db 121,32,61,32 ;; < 469 ./examples/struct_assign.cy:24:14 StringLiteral("y = ") >
	string_9 db 122,32,61,32 ;; < 493 ./examples/struct_assign.cy:25:14 StringLiteral("z = ") >
	string_10 db 119,32,61,32 ;; < 517 ./examples/struct_assign.cy:26:14 StringLiteral("w = ") >
	string_11 db 104,101,108,108,111,32,61,32 ;; < 546 ./examples/struct_assign.cy:27:18 StringLiteral("hello = ") >
	string_12 db 120,32,61,32 ;; < 621 ./examples/struct_assign.cy:33:14 StringLiteral("x = ") >
	string_13 db 121,32,61,32 ;; < 645 ./examples/struct_assign.cy:34:14 StringLiteral("y = ") >
	string_14 db 122,32,61,32 ;; < 669 ./examples/struct_assign.cy:35:14 StringLiteral("z = ") >
	string_15 db 119,32,61,32 ;; < 693 ./examples/struct_assign.cy:36:14 StringLiteral("w = ") >
	string_16 db 104,101,108,108,111,32,61,32 ;; < 722 ./examples/struct_assign.cy:37:18 StringLiteral("hello = ") >

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
	sub rsp, 32
	
	
	
	
	
	
	;; Assigning local struct MyStruct
	;; Member name: x Struct offset = 32. Member offset: 0
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], 400000000
	
	;; Member name: y Struct offset = 32. Member offset: 8
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 24], 2000000
	
	;; Member name: z Struct offset = 32. Member offset: 12
	;; assign_local_number of type Integer16
	mov WORD [rbp - 20], 42069
	
	;; Member name: w Struct offset = 32. Member offset: 14
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 18], 200
	
	mov QWORD [rbp - 8], 12
	mov QWORD [rbp - 16], string_0
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 32], type: Integer
	;; no xor here
	mov rax, [rbp - 32]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_2
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 24], type: Integer32
	xor rax, rax
	mov eax, [rbp - 24]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_3
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 20], type: Integer16
	xor rax, rax
	mov ax, [rbp - 20]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_4
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 18], type: Integer8
	xor rax, rax
	mov al, [rbp - 18]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_5
	mov rdx, 8
	syscall
	
	mov rax, 1
	mov rdi, 1
	mov rsi, [rbp - 16]
	mov rdx, [rbp - 8]
	syscall
	
	
	
	;; Plus get the two operands from the stack. Result type: Integer. Token: < 353 ./examples/struct_assign.cy:18:12 Op(Plus) >
	mov rax, 600
	mov rbx, [rbp - 32]
	add rax, rbx
	;; will lock rax. first = 600. second = [rbp - 32]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], rax
	
	
	;; Plus get the two operands from the stack. Result type: Integer32. Token: < 373 ./examples/struct_assign.cy:19:12 Op(Plus) >
	mov rax, 0 ;; clearing eax
	mov rbx, 0 ;; clearing ebx
	mov eax, 400
	mov ebx, [rbp - 24]
	add eax, ebx
	;; will lock rax. first = 400. second = [rbp - 24]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 24], eax
	
	
	;; Plus get the two operands from the stack. Result type: Integer8. Token: < 393 ./examples/struct_assign.cy:20:12 Op(Plus) >
	mov rax, 0 ;; clearing al
	mov rbx, 0 ;; clearing bl
	mov al, 20
	mov bl, [rbp - 18]
	add al, bl
	;; will lock rax. first = 20. second = [rbp - 18]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 18], al
	
	
	mov QWORD [rbp - 8], 14
	mov QWORD [rbp - 16], string_6
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_7
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 32], type: Integer
	;; no xor here
	mov rax, [rbp - 32]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_8
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 24], type: Integer32
	xor rax, rax
	mov eax, [rbp - 24]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_9
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 20], type: Integer16
	xor rax, rax
	mov ax, [rbp - 20]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_10
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 18], type: Integer8
	xor rax, rax
	mov al, [rbp - 18]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_11
	mov rdx, 8
	syscall
	
	mov rax, 1
	mov rdi, 1
	mov rsi, [rbp - 16]
	mov rdx, [rbp - 8]
	syscall
	
	
	
	mov rax, [rbp - 32]
	mov rbx, 600
	add rax, rbx
	mov [rbp - 32], rax
	
	
	mov rax, [rbp - 24]
	mov rbx, 400
	add rax, rbx
	mov [rbp - 24], rax
	
	
	mov rax, [rbp - 18]
	mov rbx, 20
	add rax, rbx
	mov [rbp - 18], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_12
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 32], type: Integer
	;; no xor here
	mov rax, [rbp - 32]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_13
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 24], type: Integer32
	xor rax, rax
	mov eax, [rbp - 24]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_14
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 20], type: Integer16
	xor rax, rax
	mov ax, [rbp - 20]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_15
	mov rdx, 4
	syscall
	
	;; get_vec_for_write_number. stack_member: [rbp - 18], type: Integer8
	xor rax, rax
	mov al, [rbp - 18]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_16
	mov rdx, 8
	syscall
	
	mov rax, 1
	mov rdi, 1
	mov rsi, [rbp - 16]
	mov rdx, [rbp - 8]
	syscall
	
	
	;; 'a' at '[rbp - 32]'
	mov rsp, rbp
	pop rbp
	ret
	

