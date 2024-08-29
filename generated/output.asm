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
	
	mov rax, string_0
	push rax
	push 12
	
	push 200
	
	push 42069
	
	push 2000000
	
	push 400000000
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 31], rax
	
	;; assign_local_number of type Integer32
	xor rax, rax
	pop rax
	mov [rbp - 23], eax
	
	;; assign_local_number of type Integer16
	xor rax, rax
	pop rax
	mov [rbp - 19], ax
	
	;; assign_local_number of type Integer8
	xor rax, rax
	pop rax
	mov [rbp - 17], al
	
	pop rbx
	pop rax
	mov [rbp - 8], rbx
	mov [rbp - 16], rax
	
	mov rax, string_1
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov rax, [rbp - 31]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_2
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov eax, [rbp - 23]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_3
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov ax, [rbp - 19]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_4
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov al, [rbp - 17]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_5
	push rax
	push 8
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	mov rax, [rbp - 16]
	push rax
	mov rax, [rbp - 8]
	push rax
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov rax, [rbp - 31]
	push rax
	
	push 600
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 31], rax
	
	xor rax, rax
	mov eax, [rbp - 23]
	push rax
	
	push 400
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; assign_local_number of type Integer32
	xor rax, rax
	pop rax
	mov [rbp - 23], eax
	
	xor rax, rax
	mov al, [rbp - 17]
	push rax
	
	push 20
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; assign_local_number of type Integer8
	xor rax, rax
	pop rax
	mov [rbp - 17], al
	
	mov rax, string_6
	push rax
	push 14
	
	pop rbx
	pop rax
	mov [rbp - 8], rbx
	mov [rbp - 16], rax
	
	mov rax, string_7
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov rax, [rbp - 31]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_8
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov eax, [rbp - 23]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_9
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov ax, [rbp - 19]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_10
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov al, [rbp - 17]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_11
	push rax
	push 8
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	mov rax, [rbp - 16]
	push rax
	mov rax, [rbp - 8]
	push rax
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	push 600
	
	mov rax, [rbp - 31]
	pop rbx
	add rax, rbx
	mov [rbp - 31], rax
	
	push 400
	
	mov rax, [rbp - 31]
	pop rbx
	add rax, rbx
	mov [rbp - 31], rax
	
	push 20
	
	mov rax, [rbp - 31]
	pop rbx
	add rax, rbx
	mov [rbp - 31], rax
	
	mov rax, string_12
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov rax, [rbp - 31]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_13
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov eax, [rbp - 23]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_14
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov ax, [rbp - 19]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_15
	push rax
	push 4
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov al, [rbp - 17]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_16
	push rax
	push 8
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	mov rax, [rbp - 16]
	push rax
	mov rax, [rbp - 8]
	push rax
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	mov rsp, rbp
	pop rbp
	ret
	

