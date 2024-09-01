%include "../generated/std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8

section .text
    global _start

_add:
	; push rbp
	; mov rbp, rsp

    mov rax, rdi
    mov rbx, rsi
    add rax, rbx

    push rax
    call _printRAX

    pop rax

	; mov rsp, rbp
	; pop rbp
	ret

_start: 
    mov rax, 69

    mov rdi, 1
    mov rsi, 4
    call _add

    call _printRAX

    mov rax, 0
    exit rax
