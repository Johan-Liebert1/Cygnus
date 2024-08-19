SYS_EXIT equ 60

%macro exit 1 ; 1 -> takes one argument
    mov rax, SYS_EXIT
    mov rdi, %1
    syscall
%endmacro

section .bss
	digitSpace resb 100
	digitSpacePos resb 8

section .data
    pi dq 3.14
    storage dq 0

section .text
    global _start

_printRAX:
    
    mov rcx, digitSpace
    mov rbx, 10 
    
    mov [rcx], rbx
    inc rcx
    
    mov [digitSpacePos], rcx

    call _printRAXLoop

    ret

_printRAXLoop:
    mov rdx, 0
    mov rbx, 10
    
    div rbx
    push rax
    add rdx, 48
    mov rcx, [digitSpacePos]
    mov [rcx], dl
    inc rcx

    mov [digitSpacePos], rcx

    pop rax
    cmp rax, 0

    jne _printRAXLoop

    dec rcx
    mov [digitSpacePos], rcx

    call _printRAXLoop2

    ret

_printRAXLoop2:
    mov rcx, [digitSpacePos]

    mov rax, 1
    mov rdi, 1
    mov rsi, rcx
    mov rdx, 1
    syscall

    mov rcx, [digitSpacePos]
    dec rcx
    mov [digitSpacePos], rcx

    cmp rcx, digitSpace
    jge _printRAXLoop2

    ret

_printString:
    mov rax, 1
    mov rdi, 1
    mov rsi, r9
    mov rdx, r8
    syscall

    ret

_start: 
    fld qword [pi]
    fld qword [pi]

    movsd xmm0, [pi]
    movsd xmm1, [pi]
    addsd xmm0, xmm1

    movsd [storage], xmm0
    mov rax, [storage]

    call _printRAX

    mov rax, 0
    exit rax
