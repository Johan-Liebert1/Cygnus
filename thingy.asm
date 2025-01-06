_main:
    push rbp
    mov rbp, rsp
    ;; Make sure that the stack pointer is 16 byte aligned always
    sub rsp, 16


    mov QWORD [rbp - 8], 5
    mov QWORD [rbp - 16], string_0


    mov rax, 1


    mov rdi, 1

    mov rsi, [rbp - 8]

    ;; Saving register rax's value
    push rax

    ;; Saving register rsi's value
    push rsi

    ;; Saving register rdi's value
    push rdi

    lea rax, [rbp - 16]

    ;; Moving argument number 1
    mov rdi, rax

    call _get_len

    ;; popping saved register value into rdi
    pop rdi
    ;; popping saved register value into rsi
    pop rsi
    ;; popping saved register value into rax
    pop rax
    mov rbx, rax

    mov rdx, rax

    syscall
