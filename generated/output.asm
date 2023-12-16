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
	jmp .skip_next0
	.skip_0:
	mov rax, 1
	.skip_next0:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	.if:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end
	push 9
	push 1
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	call _printRAX
	jmp .else_end
	.if_end:
	push 6
	push 9
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jg .skip_1
	mov rax, 0
	jmp .skip_next1
	.skip_1:
	mov rax, 1
	.skip_next1:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	.elif_0:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_end
	push 10
	push 1
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	call _printRAX
	jmp .else_end
	.elif_0_end:
	push 1
	push 0
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jl .skip_2
	mov rax, 0
	jmp .skip_next2
	.skip_2:
	mov rax, 1
	.skip_next2:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	.elif_1:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else
	push 68
	push 1
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	call _printRAX
	jmp .else_end
	.elif_1_end:
	.else:
	push 420
	push 0
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	call _printRAX
	.else_end:
	exit 0

