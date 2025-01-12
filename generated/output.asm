%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	;; For floating point operations
	float_imm dq 0

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
	sub rsp, 18
	
	
	
	
	;; Multiply get the two operands from the stack. Type: Integer
	xor rdx, rdx
	mov rax, 7
	mov rbx, 5
	mul rbx
	
	;; Plus get the two operands from the stack. Result type: Integer8. Token: < 36 ./tests/first.cy:2:18 Op(Plus) >
	mov rbx, rax
	mov rcx, 9
	add rbx, rcx
	;; will lock rbx. first = rax. second = 9. Locked: [rax, rbx, rcx]
	
	;; assign_local_number of type Integer8
	mov BYTE [rbp - 1], bl
	
	;; 'a' at '[rbp - 1]'
	mov rsp, rbp
	pop rbp
	ret
	

