%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .data
	string_0 db 32,105,115,32,97,32,112,97,108,105,110,100,114,111,109,101,10
	string_1 db 114,97,99,101,99,97,114

section .text
	global _start

_start:
    xor rax, rax
    mov rax, 2

    mov [rsp - 3], ax
    xor rax, rax
    mov ax, [rsp - 3]

    call _printRAX

	exit 0
