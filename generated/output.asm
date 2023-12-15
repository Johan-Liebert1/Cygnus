%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8

section .text
	global _start

_start:
push 5
	push 4
	;; get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	;; push the result back onto the stack
	push rax
	push 9
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jge .skip
	mov rax, 0
	jmp .skip_n
	.skip:
	mov rax, 1
	.skip_n:
	call _printRAX
	exit 0

