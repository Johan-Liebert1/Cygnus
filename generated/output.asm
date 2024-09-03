%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 72,101,108,108,111,32,87,111,114,108,100,10
	string_1 db 120,32,61,32
	string_2 db 121,32,61,32
	string_3 db 122,32,61,32
	string_4 db 119,32,61,32
	string_5 db 104,101,108,108,111,32,61,32
	string_6 db 71,111,111,100,98,121,101,32,87,111,114,108,100,10
	string_7 db 120,32,61,32
	string_8 db 121,32,61,32
	string_9 db 122,32,61,32
	string_10 db 119,32,61,32
	string_11 db 104,101,108,108,111,32,61,32
	string_12 db 120,32,61,32
	string_13 db 121,32,61,32
	string_14 db 122,32,61,32
	string_15 db 119,32,61,32
	string_16 db 104,101,108,108,111,32,61,32

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
	sub rsp, 46
	
	
	
	
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 31], 400000000
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 23], 2000000
	
	;; assign_local_number of type Integer16
	mov WORD [rbp - 19], 42069
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 17], 200
	
	mov QWORD [rbp - 8], 12
	mov QWORD [rbp - 16], string_0
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 31]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_2
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 23]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_3
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 19]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_4
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 17]
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
	
	
	;; Plus get the two operands from the stack
	mov rax, 600
	mov rbx, [rbp - 31]
	add rax, rbx
	;; will lock rax. first = 600. second = [rbp - 31]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 31], rax
	
	
	;; Plus get the two operands from the stack
	mov rax, 400
	mov rbx, [rbp - 23]
	add rax, rbx
	;; will lock rax. first = 400. second = [rbp - 23]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 23], eax
	
	
	;; Plus get the two operands from the stack
	mov rax, 20
	mov rbx, [rbp - 17]
	add rax, rbx
	;; will lock rax. first = 20. second = [rbp - 17]. Locked: [rax, rbx]
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 17], al
	
	
	mov QWORD [rbp - 8], 14
	mov QWORD [rbp - 16], string_6
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_7
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 31]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_8
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 23]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_9
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 19]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_10
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 17]
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
	
	
	mov rax, [rbp - 31]
	mov rbx, 600
	add rax, rbx
	mov [rbp - 31], rax
	
	
	mov rax, [rbp - 23]
	mov rbx, 400
	add rax, rbx
	mov [rbp - 23], rax
	
	
	mov rax, [rbp - 17]
	mov rbx, 20
	add rax, rbx
	mov [rbp - 17], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_12
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 31]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_13
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 23]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_14
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 19]
	call _printRAX
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_15
	mov rdx, 4
	syscall
	
	mov rax, [rbp - 17]
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
	
	mov rsp, rbp
	pop rbp
	ret
	

