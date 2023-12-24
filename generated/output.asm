%include "std.asm"

section .bss
digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
string_0 db 109,121,95,119,111,114,108,100
	string_1 db 121,111,117,114,95,119,111,114,108,100

section .text
global _start

_start:
mov [argc], rsp
	call _my_world
	call _your_world
	exit 0

_my_world:
mov rax, string_0
	push rax
	push 8
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	ret

_your_world:
mov rax, string_1
	push rax
	push 10
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	ret

