%include "std.asm"

section .bss
digitSpace resb 100
	digitSpacePos resb 8

section .data
string_0 db 104,101,108,108,111,92,110

section .text
global _start

_start:
mov rax, string_0
	push rax
	push 7
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	exit 0

