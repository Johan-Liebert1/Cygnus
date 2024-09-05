%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 119,111,119,10

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
	sub rsp, 48
	
	
	
	;; Assigning local struct A
	
	;; Member name: val Struct offset = 32. Member offset: 0
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 32], 69
	
	mov QWORD [rbp - 8], 4
	mov QWORD [rbp - 16], string_0
	
	;; Storing address of struct A for variable a not in handle_local_ptr
	lea rax, [rbp - 32]
	
	;; assign_local_pointer of type A
	mov rbx, rax
	mov [rbp - 40], rax
	
	;; Handling ptr to struct for Ptr -> A
	
	mov rbx, [rbp - 40]
	add rbx, 0
	xor rax, rax
	mov rax, [rbx]
	
	call _printRAX
	
	
	;; Handling ptr to struct for Ptr -> A
	
	mov rbx, [rbp - 40]
	add rbx, 16
	xor rax, rax
	mov rax, [rbx]
	mov rcx, [rbx + 8]
	
	mov rax, 1
	mov rdi, 1
	mov rsi, rax
	mov rdx, rcx
	syscall
	
	
	mov rsp, rbp
	pop rbp
	ret
	

