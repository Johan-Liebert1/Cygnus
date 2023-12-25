%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	string_0 db 104,101,108,108,111,10
	string_1 db 104,105,10
	string_2 db 98,121,101,10
	string_3 db 52,32,62,32,50,10

section .text
	global _start

_start:
	mov [argc], rsp
	push 1
	push 5
	push 1
	.loop_0:
	pop rcx
	pop rbx
	pop rax
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	inc rax
	inc rbx
	push rax
	push rbx
	push rcx
	push 5
	push 8
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_0
	mov rax, 0
	jmp .skip_next0
	.skip_0:
	mov rax, 1
	.skip_next0:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	.if_0:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_0
	mov rax, string_0
	push rax
	push 6
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	jmp .else_end_0
	.if_end_0:
	push 6
	push 2
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_1
	mov rax, 0
	jmp .skip_next1
	.skip_1:
	mov rax, 1
	.skip_next1:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	.elif_0_0:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_0
	mov rax, string_1
	push rax
	push 3
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	jmp .else_end_0
	.elif_0_0_end:
	.else_0:
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
	.else_end_0:
	jmp .loop_0
	.loop_end_0:
	push 1
	push 3
	push 1
	.loop_1:
	pop rcx
	pop rbx
	pop rax
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_1
	inc rax
	inc rbx
	push rax
	push rbx
	push rcx
	push 4
	push 2
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jg .skip_2
	mov rax, 0
	jmp .skip_next2
	.skip_2:
	mov rax, 1
	.skip_next2:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	.if_1:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_1
	mov rax, string_3
	push rax
	push 6
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	jmp .if_end_1
	.if_end_1:
	jmp .loop_1
	.loop_end_1:
	exit 0

