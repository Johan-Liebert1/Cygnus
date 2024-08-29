%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8
	event resb 256

section .data
	;; For floating point operations
	float_imm dq 0
	extern SDL_Init
	extern SDL_CreateWindow
	extern SDL_PollEvent
	extern SDL_GetTicks
	extern SDL_GetError
	extern SDL_Quit
	extern SDL_CreateRenderer
	extern SDL_SetRenderDrawColor
	extern SDL_RenderFillRect
	extern SDL_RenderPresent
	extern SDL_DestroyRenderer
	extern SDL_RenderClear
	SDL_QUIT dq 0
	SDL_RENDERER_SOFTWARE dq 0
	SDL_RENDERER_ACCELERATED dq 0
	SDL_RENDERER_PRESENTVSYNC dq 0
	SDL_RENDERER_TARGETTEXTURE dq 0
	string_0 db 115,100,108,95,105,110,105,116,32,61,32
	string_1 db 104,101,108,108,111,0
	string_2 db 119,105,110,100,111,119,32,61,32
	string_3 db 114,101,110,100,101,114,101,114,32,61,32

section .text
	global _start

_start:
	mov [argc], rsp
	push 256
	
	pop rax
	mov [SDL_QUIT], rax
	
	push 1
	
	pop rax
	mov [SDL_RENDERER_SOFTWARE], rax
	
	push 2
	
	pop rax
	mov [SDL_RENDERER_ACCELERATED], rax
	
	push 4
	
	pop rax
	mov [SDL_RENDERER_PRESENTVSYNC], rax
	
	push 8
	
	pop rax
	mov [SDL_RENDERER_TARGETTEXTURE], rax
	
	call _main
	
	exit 0

_gol:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 400
	
	mov rsp, rbp
	pop rbp
	ret
	

_main:
	push rbp
	mov rbp, rsp
	;; Make sure that the stack pointer is 16 byte aligned always
	sub rsp, 144
	
	push 32
	
	pop rdi
	call SDL_Init
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 8], rax
	
	mov rax, string_0
	push rax
	push 11
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	xor rax, rax
	mov rax, [rbp - 8]
	push rax
	
	pop rax
	call _printRAX
	
	mov rax, string_1
	push rax
	push 6
	
	pop rbx
	pop rax
	mov [rbp - 16], rbx
	mov [rbp - 24], rax
	
	push 8196
	
	push 600
	
	push 800
	
	push 10
	
	push 10
	
	mov rax, [rbp - 24]
	push rax
	
	pop rdi
	pop rsi
	pop rdx
	pop rcx
	pop r8
	pop r9
	call SDL_CreateWindow
	push rax
	
	;; assign_local_pointer of type Integer
	pop rax
	mov [rbp - 32], rax
	
	mov rax, string_2
	push rax
	push 9
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	mov rax, [rbp - 32]
	push rax
	
	pop rax
	call _printRAX
	
	push 0
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 40], rax
	
	mov rax, [rbp - 32]
	push rax
	
	mov rax, [rbp - 40]
	push rax
	
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
	
	call SDL_GetError
	push rax
	
	;; assign_local_pointer of type Character
	pop rax
	mov [rbp - 48], rax
	
	mov rax, [rbp - 48]
	push rax
	
	pop rax
	call _printRAX
	
	pop rax
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_0
	.if_end_0:
	
	mov eax, [SDL_RENDERER_ACCELERATED]
	push rax
	
	push 0
	
	mov rax, [rbp - 32]
	push rax
	
	pop rdi
	pop rsi
	pop rdx
	call SDL_CreateRenderer
	push rax
	
	;; assign_local_pointer of type Integer
	pop rax
	mov [rbp - 56], rax
	
	mov rax, string_3
	push rax
	push 11
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	mov rax, [rbp - 56]
	push rax
	
	pop rax
	call _printRAX
	
	push 60
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 64], rax
	
	push 0
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 72], rax
	
	call SDL_GetTicks
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 80], rax
	
	call SDL_GetTicks
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 88], rax
	
	push 0
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 96], rax
	
	push 0
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 104], rax
	
	push 50
	
	push 50
	
	push 100
	
	push 100
	
	push 45
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; assign_local_number of type Integer32
	xor rax, rax
	pop rax
	mov [rbp - 120], eax
	
	;; assign_local_number of type Integer32
	xor rax, rax
	pop rax
	mov [rbp - 116], eax
	
	;; assign_local_number of type Integer32
	xor rax, rax
	pop rax
	mov [rbp - 112], eax
	
	;; assign_local_number of type Integer32
	xor rax, rax
	pop rax
	mov [rbp - 108], eax
	
	.loop_0:
	
	xor rax, rax
	mov rax, [rbp - 72]
	push rax
	
	push 1
	
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
	
	.if_1:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_1
	
	;; --- break ----
	jmp .loop_end_0
	
	jmp .if_end_1
	.if_end_1:
	
	call SDL_GetTicks
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 80], rax
	
	xor rax, rax
	mov rax, [rbp - 80]
	push rax
	
	xor rax, rax
	mov rax, [rbp - 88]
	push rax
	
	;; Minus get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	push rax
	
	mov rax, [rbp - 96]
	pop rbx
	add rax, rbx
	mov [rbp - 96], rax
	
	xor rax, rax
	mov rax, [rbp - 96]
	push rax
	
	pop rax
	call _printRAX
	
	xor rax, rax
	mov rax, [rbp - 96]
	push rax
	
	xor rax, rax
	mov rax, [rbp - 64]
	push rax
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jl .skip_2
	mov rax, 0
	jmp .skip_next2
	.skip_2:
	mov rax, 1
	.skip_next2:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	.if_2:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_2
	
	;; --- continue ----
	jmp .loop_0_end_start
	
	jmp .if_end_2
	.if_end_2:
	
	push 255
	
	push 0
	
	push 0
	
	push 0
	
	mov rax, [rbp - 56]
	push rax
	
	pop rdi
	pop rsi
	pop rdx
	pop rcx
	pop r8
	call SDL_SetRenderDrawColor
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 104], rax
	
	mov rax, [rbp - 56]
	push rax
	
	pop rdi
	call SDL_RenderClear
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 104], rax
	
	xor rax, rax
	mov rax, event
	push rax
	
	pop rdi
	call SDL_PollEvent
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 104], rax
	
	xor rax, rax
	mov rax, [rbp - 104]
	push rax
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	jne .skip_3
	mov rax, 0
	jmp .skip_next3
	.skip_3:
	mov rax, 1
	.skip_next3:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	.if_3:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_3
	
	;; Dereferencing variable event. handle_global_ptr
	mov rbx, event
	mov rbx, [rbx]
	xor rax, rax
	mov eax, ebx
	push rax
	;; Finish dereferencing variable event
	
	mov eax, [SDL_QUIT]
	push rax
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_4
	mov rax, 0
	jmp .skip_next4
	.skip_4:
	mov rax, 1
	.skip_next4:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	.if_4:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_4
	
	push 1
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 72], rax
	
	jmp .if_end_4
	.if_end_4:
	
	jmp .if_end_3
	.if_end_3:
	
	push 255
	
	push 0
	
	push 0
	
	push 255
	
	mov rax, [rbp - 56]
	push rax
	
	pop rdi
	pop rsi
	pop rdx
	pop rcx
	pop r8
	call SDL_SetRenderDrawColor
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 104], rax
	
	;; Storing address of struct SDL_Rect for variable rect not in handle_local_ptr
	lea rax, [rbp - 120]
	push rax
	
	mov rax, [rbp - 56]
	push rax
	
	pop rdi
	pop rsi
	call SDL_RenderFillRect
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 104], rax
	
	push 1
	
	mov rax, [rbp - 120]
	pop rbx
	add rax, rbx
	mov [rbp - 120], rax
	
	push 1
	
	mov rax, [rbp - 120]
	pop rbx
	add rax, rbx
	mov [rbp - 120], rax
	
	mov rax, [rbp - 56]
	push rax
	
	pop rdi
	call SDL_RenderPresent
	
	call SDL_GetTicks
	push rax
	
	;; assign_local_number of type Integer
	xor rax, rax
	pop rax
	mov [rbp - 88], rax
	
	.loop_0_end_start:
	jmp .loop_0
	.loop_end_0:
	
	mov rax, [rbp - 56]
	push rax
	
	pop rdi
	call SDL_DestroyRenderer
	
	call SDL_Quit
	
	mov rsp, rbp
	pop rbp
	ret
	

