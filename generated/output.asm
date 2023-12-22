%include "std.asm"

section .bss
digitSpace resb 100
	digitSpacePos resb 8

section .text
global _start

_start:
push 5
	push 7
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jne .skip_0
	mov rax, 0
	jmp .skip_next0
	.skip_0:
	mov rax, 1
	.skip_next0:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	pop rax
	call _printRAX
	exit 0

