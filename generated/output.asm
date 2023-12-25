%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .text
	global _start

_start:
	mov [argc], rsp
	push 3
	push 2
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 4
	push 2
	push 3
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 5
	push 3
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	push 2
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 1
	push 1
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 0
	push 3
	push 2
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 9
	push 5
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 10
	push 3
	push 2
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 15
	push 5
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	push 3
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 6
	push 1
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 8
	push 3
	push 1
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 3
	push 2
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 2
	push 3
	push 2
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 5
	push 3
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	push 2
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 4
	push 3
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 5
	push 2
	push 2
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 8
	push 2
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 18
	push 2
	push 1
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 12
	push 3
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	push 2
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 15
	push 5
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 20
	push 5
	push 1
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 5
	push 3
	push 2
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 6
	push 2
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	push 3
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	push 6
	push 2
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 12
	push 3
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	push 3
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	push 2
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 5
	push 2
	push 3
	push 1
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 5
	push 3
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	push 2
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	push 4
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	push 3
	push 4
	push 6
	push 5
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	push 7
	push 8
	push 9
	push 10
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	push 11
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	push 12
	push 13
	;; get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	push 1434
	push 15
	push 142
	push 16
	;; get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	;; clean up rdx as this might mess up the final output
	xor rdx, rdx
	;; get the two operands from the stack
	pop rbx
	pop rax
	div rbx
	;; push the result back onto the stack
	push rax
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	pop rax
	call _printRAX
	exit 0

