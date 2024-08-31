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
	
	
	;; Plus get the two operands from the stack
	mov rax, 2
	mov rbx, 3
	add rax, rbx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 3
	mov rbx, 2
	add rax, rbx
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rcx, 4
	add rbx, rcx
	
	mov rax, rbx
	call _printRAX
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 3
	mov rbx, 5
	add rax, rbx
	
	
	;; Plus get the two operands from the stack
	mov rbx, 2
	mov rcx, rax
	add rbx, rcx
	
	mov rax, rbx
	call _printRAX
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 1
	mov rbx, 1
	add rax, rbx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 2
	mov rbx, 3
	add rax, rbx
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rcx, 0
	add rbx, rcx
	
	mov rax, rbx
	call _printRAX
	
	
	
	;; Minus get the two operands from the stack
	mov rbx, 5
	mov rax, 9
	sub rax, rbx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 2
	mov rbx, 3
	add rax, rbx
	
	;; Minus get the two operands from the stack
	mov rcx, rax
	mov rbx, 10
	sub rbx, rcx
	
	mov rax, rbx
	call _printRAX
	
	
	
	;; Minus get the two operands from the stack
	mov rbx, 5
	mov rax, 15
	sub rax, rbx
	
	
	;; Minus get the two operands from the stack
	mov rcx, 3
	mov rbx, rax
	sub rbx, rcx
	
	mov rax, rbx
	call _printRAX
	
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, 6
	sub rax, rbx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 1
	mov rbx, 3
	add rax, rbx
	
	;; Minus get the two operands from the stack
	mov rcx, rax
	mov rbx, 8
	sub rbx, rcx
	
	mov rax, rbx
	call _printRAX
	
	
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 2
	mov rbx, 3
	mul rbx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 2
	mov rbx, 3
	mul rbx
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, rbx
	mov rcx, 2
	mul rcx
	
	mov rax, rax
	call _printRAX
	
	
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 3
	mov rbx, 5
	mul rbx
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 2
	mov rcx, rbx
	mul rcx
	
	mov rax, rax
	call _printRAX
	
	
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 3
	mov rbx, 4
	mul rbx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 2
	mov rbx, 2
	mul rbx
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, rbx
	mov rcx, 5
	mul rcx
	
	mov rax, rax
	call _printRAX
	
	
	
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rbx, 2
	mov rax, 8
	div rbx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 1
	mov rbx, 2
	add rax, rbx
	
	mov rbx, rax
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rcx, rbx
	mov rax, 18
	div rcx
	
	mov rax, rax
	call _printRAX
	
	
	
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rbx, 3
	mov rax, 12
	div rbx
	
	
	mov rbx, rax
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rcx, 2
	mov rax, rbx
	div rcx
	
	mov rax, rax
	call _printRAX
	
	
	
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rbx, 5
	mov rax, 15
	div rbx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 1
	mov rbx, 5
	add rax, rbx
	
	mov rbx, rax
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rcx, rbx
	mov rax, 20
	div rcx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 2
	mov rbx, 3
	mul rbx
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rcx, 5
	add rbx, rcx
	
	mov rax, rbx
	call _printRAX
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 2
	mov rbx, 6
	add rax, rbx
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 3
	mov rcx, rbx
	mul rcx
	
	
	
	mov rbx, rax
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rcx, 2
	mov rax, 6
	div rcx
	
	;; Minus get the two operands from the stack
	mov rdx, rax
	mov rcx, rbx
	sub rcx, rdx
	
	mov rax, rcx
	call _printRAX
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 3
	mov rbx, 12
	add rax, rbx
	
	
	mov rbx, rax
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rcx, 3
	mov rax, rbx
	div rcx
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 2
	mov rcx, rbx
	mul rcx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	
	;; Minus get the two operands from the stack
	mov rbx, 1
	mov rax, 3
	sub rax, rbx
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, rbx
	mov rcx, 2
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rcx, 5
	add rbx, rcx
	
	mov rax, rbx
	call _printRAX
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 3
	mov rbx, 5
	add rax, rbx
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 2
	mov rcx, rbx
	mul rcx
	
	
	mov rbx, rax
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rcx, 4
	mov rax, rbx
	div rcx
	
	mov rax, rax
	call _printRAX
	
	
	
	
	
	;; Minus get the two operands from the stack
	mov rbx, 5
	mov rax, 6
	sub rax, rbx
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, rbx
	mov rcx, 4
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rcx, 3
	add rbx, rcx
	
	
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 10
	mov rcx, 9
	add rax, rcx
	
	mov rcx, rax
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rsi, rcx
	mov rax, 8
	div rsi
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, 7
	add rcx, rdx
	
	
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rsi, 11
	mov rax, rcx
	div rsi
	
	mov rcx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, rcx
	mov rsi, rbx
	mul rsi
	
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 13
	mov rcx, 12
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	
	
	
	
	
	;; Minus get the two operands from the stack
	mov rbx, 16
	mov rax, 142
	sub rax, rbx
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rdx, 15
	add rbx, rdx
	
	;; Divide clean up rdx as this might mess up the final output as 'div' stores quotient in 'rax' and remainder in 'rdx'
	xor rdx, rdx
	;; get the two operands from the stack
	mov rsi, rbx
	mov rax, 1434
	div rsi
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rdx, rcx
	add rbx, rdx
	
	mov rax, rbx
	call _printRAX
	
	exit 0

