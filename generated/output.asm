%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8

section .text
	global _start

_start:
push 5
	push 8
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jg .skip_0
	mov rax, 0
	jmp .skip_n0
	.skip_0:
	mov rax, 1
	.skip_n0:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	exit 0

.if:
pop rcx
	;; if the comparison value is false, jump to the next label altogether
	jz .elif_0
	push 9
	call _printRAX
	.if_end:
	push 6
	push 9
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jl .skip_1
	mov rax, 0
	jmp .skip_n1
	.skip_1:
	mov rax, 1
	.skip_n1:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax

.elif_0:
pop rcx
	;; if the comparison value is false, jump to the next label altogether
	jz .elif_0_end
	push 10
	push 1
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	call _printRAX
	.elif_0_end:

