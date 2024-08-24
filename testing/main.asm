SYS_EXIT equ 60

%macro exit 1 ; 1 -> takes one argument
    mov rax, SYS_EXIT
    mov rdi, %1
    syscall
%endmacro

section .text
    extern print_thingy
    global _start

_start: 
    call print_thingy

    mov rax, 0
    exit rax
