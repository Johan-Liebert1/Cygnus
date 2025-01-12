%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8
	lower_char_occurances resb 208

section .data
	;; For floating point operations
	float_imm dq 0
	string_0 db 108,111,119,101,114,95,99,104,97,114,95,111,99,99,117,114,97,110,99,101,115,32,61,32
	string_1 db 97,100,100,114,95,116,111,95,117,112,100,97,116,101,32,61,32

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
	sub rsp, 16
	
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 1], 3
	
	;; Global new thingy
	xor rax, rax
	mov rax, lower_char_occurances
	
	
	mov rbx, rax ;; Saving rax
	;; Multiply get the two operands from the stack. Type: Integer
	xor rdx, rdx
	mov rax, 0 ;; clearing al
	mov rcx, 0 ;; clearing cl
	mov al, 8
	mov cl, [rbp - 1]
	mul rcx
	
	;; Plus get the two operands from the stack. Result type: Pointer -> Integer. Token: < 125 ./tests/first.cy:5:51 Op(Plus) >
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = rax. second = rbx. Locked: [rbx, rax, rcx, rdx]
	
	;; src/asm/variable_assignment.rs:236 assign_local_pointer var 'addr_to_update' of type Integer. times_dereferenced: 0
	mov rax, rcx
	mov [rbp - 16], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_0
	mov rdx, 24
	syscall
	
	;; Global new thingy
	xor rax, rax
	mov rax, lower_char_occurances
	
	call _printRAX
	
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_1
	mov rdx, 17
	syscall
	
	mov rax, [rbp - 16]
	
	call _printRAX
	
	
	;; 'addr_to_update' at '[rbp - 16]'
	;; 'inc' at '[rbp - 1]'
	mov rsp, rbp
	pop rbp
	ret
	

