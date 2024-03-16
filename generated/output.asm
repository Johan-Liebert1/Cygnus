%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .text
	global _start

_start:
	mov [argc], rsp
	call _main
	
	exit 0

_main:
	push rbp
	mov rbp, rsp
	sub rsp, 216
	
	push 10
	
	push 7
	
	push 15
	
	push 14
	
	push 2
	
	push 6
	
	push 11
	
	push 1
	
	push 3
	
	push 4
	
	push 5
	
	push 13
	
	push 9
	
	push 8
	
	push 12
	
	pop rax
	mov [rbp - 8], rax
	pop rax
	mov [rbp - 16], rax
	pop rax
	mov [rbp - 24], rax
	pop rax
	mov [rbp - 32], rax
	pop rax
	mov [rbp - 40], rax
	pop rax
	mov [rbp - 48], rax
	pop rax
	mov [rbp - 56], rax
	pop rax
	mov [rbp - 64], rax
	pop rax
	mov [rbp - 72], rax
	pop rax
	mov [rbp - 80], rax
	pop rax
	mov [rbp - 88], rax
	pop rax
	mov [rbp - 96], rax
	pop rax
	mov [rbp - 104], rax
	pop rax
	mov [rbp - 112], rax
	pop rax
	mov [rbp - 120], rax
	
	push 0
	
	pop rax
	mov [rbp - 128], rax
	
	push 0
	
	pop rax
	mov [rbp - 136], rax
	
	push 0
	
	push 15
	
	push 1
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 160], rcx
	mov [rbp - 152], rbx
	mov [rbp - 144], rax
	.loop_0:
	mov rcx, [rbp - 160]
	mov rbx, [rbp - 152]
	mov rax, [rbp - 144]
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	inc rax
	inc rbx
	mov [rbp - 152], rbx
	mov [rbp - 144], rax
	
	push 0
	
	pop rax
	mov [rbp - 136], rax
	
	push 0
	
	push 14
	
	push 1
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 184], rcx
	mov [rbp - 176], rbx
	mov [rbp - 168], rax
	.loop_1:
	mov rcx, [rbp - 184]
	mov rbx, [rbp - 176]
	mov rax, [rbp - 168]
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_1
	inc rax
	inc rbx
	mov [rbp - 176], rbx
	mov [rbp - 168], rax
	
	mov rax, [rbp - 136]
	push rax
	
	;; Start array index access
	pop rax
	mov rbx, 8
	mul rbx
	mov rbx, rbp
	sub rbx, rax
	mov rax, [rbx - 8]
	push rax
	
	mov rax, [rbp - 136]
	push rax
	
	push 1
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; Start array index access
	pop rax
	mov rbx, 8
	mul rbx
	mov rbx, rbp
	sub rbx, rax
	mov rax, [rbx - 8]
	push rax
	
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
	
	.if_0:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_0
	
	mov rax, [rbp - 136]
	push rax
	
	;; Start array index access
	pop rax
	mov rbx, 8
	mul rbx
	mov rbx, rbp
	sub rbx, rax
	mov rax, [rbx - 8]
	push rax
	
	pop rax
	mov [rbp - 192], rax
	
	mov rax, [rbp - 136]
	push rax
	
	push 1
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; Start array index access
	pop rax
	mov rbx, 8
	mul rbx
	mov rbx, rbp
	sub rbx, rax
	mov rax, [rbx - 8]
	push rax
	
	mov rax, [rbp - 136]
	push rax
	
	;; rbx stores the index, rcx has the actual value
	pop rbx
	pop rcx
	mov rax, 8
	mul rbx
	mov rbx, rbp
	sub rbx, rax
	mov [rbx - 8], rcx
	
	mov rax, [rbp - 192]
	push rax
	
	mov rax, [rbp - 136]
	push rax
	
	push 1
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; rbx stores the index, rcx has the actual value
	pop rbx
	pop rcx
	mov rax, 8
	mul rbx
	mov rbx, rbp
	sub rbx, rax
	mov [rbx - 8], rcx
	
	jmp .if_end_0
	.if_end_0:
	
	mov rax, [rbp - 136]
	push rax
	
	push 1
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 136], rax
	
	jmp .loop_1
	.loop_end_1:
	
	mov rax, [rbp - 128]
	push rax
	
	push 1
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 128], rax
	
	jmp .loop_0
	.loop_end_0:
	
	push 0
	
	pop rax
	mov [rbp - 128], rax
	
	push 0
	
	push 15
	
	push 1
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 216], rcx
	mov [rbp - 208], rbx
	mov [rbp - 200], rax
	.loop_2:
	mov rcx, [rbp - 216]
	mov rbx, [rbp - 208]
	mov rax, [rbp - 200]
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_2
	inc rax
	inc rbx
	mov [rbp - 208], rbx
	mov [rbp - 200], rax
	
	mov rax, [rbp - 128]
	push rax
	
	;; Start array index access
	pop rax
	mov rbx, 8
	mul rbx
	mov rbx, rbp
	sub rbx, rax
	mov rax, [rbx - 8]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, [rbp - 128]
	push rax
	
	push 1
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 128], rax
	
	jmp .loop_2
	.loop_end_2:
	
	mov rsp, rbp
	pop rbp
	ret
	

