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
	SDL_KEYDOWN dq 0
	SDL_ESC dq 0
	SDLK_a dq 0
	SDLK_b dq 0
	SDLK_c dq 0
	SDLK_d dq 0
	SDLK_e dq 0
	SDLK_f dq 0
	SDLK_g dq 0
	SDLK_h dq 0
	SDLK_i dq 0
	SDLK_j dq 0
	SDLK_k dq 0
	SDLK_l dq 0
	SDLK_m dq 0
	SDLK_n dq 0
	SDLK_o dq 0
	SDLK_p dq 0
	SDLK_q dq 0
	SDLK_r dq 0
	SDLK_s dq 0
	SDLK_t dq 0
	SDLK_u dq 0
	SDLK_v dq 0
	SDLK_w dq 0
	SDLK_x dq 0
	SDLK_y dq 0
	SDLK_z dq 0
	SDL_RENDERER_SOFTWARE dq 0
	SDL_RENDERER_ACCELERATED dq 0
	SDL_RENDERER_PRESENTVSYNC dq 0
	SDL_RENDERER_TARGETTEXTURE dq 0
	RECT_SPEED dq 0
	string_0 db 115,100,108,95,105,110,105,116,32,61,32
	string_1 db 104,101,108,108,111,0
	string_2 db 119,105,110,100,111,119,32,61,32
	string_3 db 114,101,110,100,101,114,101,114,32,61,32

section .text
	global _start

_start:
	mov [argc], rsp
	
	mov rax, 256
	mov [SDL_QUIT], rax
	
	
	mov rax, 768
	mov [SDL_KEYDOWN], rax
	
	
	mov rax, 27
	mov [SDL_ESC], rax
	
	
	mov rax, 97
	mov [SDLK_a], rax
	
	
	mov rax, 98
	mov [SDLK_b], rax
	
	
	mov rax, 99
	mov [SDLK_c], rax
	
	
	mov rax, 100
	mov [SDLK_d], rax
	
	
	mov rax, 101
	mov [SDLK_e], rax
	
	
	mov rax, 102
	mov [SDLK_f], rax
	
	
	mov rax, 103
	mov [SDLK_g], rax
	
	
	mov rax, 104
	mov [SDLK_h], rax
	
	
	mov rax, 105
	mov [SDLK_i], rax
	
	
	mov rax, 106
	mov [SDLK_j], rax
	
	
	mov rax, 107
	mov [SDLK_k], rax
	
	
	mov rax, 108
	mov [SDLK_l], rax
	
	
	mov rax, 109
	mov [SDLK_m], rax
	
	
	mov rax, 110
	mov [SDLK_n], rax
	
	
	mov rax, 111
	mov [SDLK_o], rax
	
	
	mov rax, 112
	mov [SDLK_p], rax
	
	
	mov rax, 113
	mov [SDLK_q], rax
	
	
	mov rax, 114
	mov [SDLK_r], rax
	
	
	mov rax, 115
	mov [SDLK_s], rax
	
	
	mov rax, 116
	mov [SDLK_t], rax
	
	
	mov rax, 117
	mov [SDLK_u], rax
	
	
	mov rax, 118
	mov [SDLK_v], rax
	
	
	mov rax, 119
	mov [SDLK_w], rax
	
	
	mov rax, 120
	mov [SDLK_x], rax
	
	
	mov rax, 121
	mov [SDLK_y], rax
	
	
	mov rax, 122
	mov [SDLK_z], rax
	
	
	mov rax, 1
	mov [SDL_RENDERER_SOFTWARE], rax
	
	
	mov rax, 2
	mov [SDL_RENDERER_ACCELERATED], rax
	
	
	mov rax, 4
	mov [SDL_RENDERER_PRESENTVSYNC], rax
	
	
	mov rax, 8
	mov [SDL_RENDERER_TARGETTEXTURE], rax
	
	
	mov rax, 10
	mov [RECT_SPEED], rax
	
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
	sub rsp, 160
	
	
	;; Moving argument number 1
	mov rdi, 32
	
	call SDL_Init
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 8], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_0
	mov rdx, 11
	syscall
	
	mov rax, QWORD [rbp - 8]
	call _printRAX
	
	
	mov QWORD [rbp - 24], 6
	mov QWORD [rbp - 32], string_1
	
	mov rax, [rbp - 32]
	
	;; Moving argument number 1
	mov rdi, rax
	
	
	;; Moving argument number 2
	mov rsi, 10
	
	
	;; Moving argument number 3
	mov rdx, 10
	
	
	;; Moving argument number 4
	mov rcx, 800
	
	
	;; Moving argument number 5
	mov r8, 600
	
	
	;; Moving argument number 6
	mov r9, 8196
	
	call SDL_CreateWindow
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 40], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_2
	mov rdx, 9
	syscall
	
	mov rax, [rbp - 40]
	
	call _printRAX
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 48], 0
	
	mov rax, [rbp - 40]
	
	mov rbx, [rbp - 48]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rbx
	mov rcx, rax
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_0:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_0
	
	call SDL_GetError
	
	;; assign_local_pointer of type Character
	mov rbx, rax
	mov [rbp - 56], rax
	
	mov rax, [rbp - 56]
	
	call _printRAX
	
	mov rsp, rbp
	pop rbp
	ret
	
	jmp .if_end_0
	.if_end_0:
	
	mov rax, [rbp - 40]
	
	;; Moving argument number 1
	mov rdi, rax
	
	
	;; Moving argument number 2
	mov rsi, 0
	
	mov eax, [SDL_RENDERER_ACCELERATED]
	
	;; Moving argument number 3
	mov rdx, rax
	
	call SDL_CreateRenderer
	
	;; assign_local_pointer of type Integer
	mov rbx, rax
	mov [rbp - 64], rax
	
	
	mov rax, 1
	mov rdi, 1
	mov rsi, string_3
	mov rdx, 11
	syscall
	
	mov rax, [rbp - 64]
	
	call _printRAX
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 72], 60
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 80], 0
	
	call SDL_GetTicks
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 88], rax
	
	call SDL_GetTicks
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 96], rax
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 104], 0
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 112], 0
	
	
	
	
	
	
	;; Plus get the two operands from the stack
	mov rax, 45
	mov rbx, 100
	add rax, rbx
	;; will lock rax. first = 45. second = 100. Locked: [rax, rbx]
	
	;; Assigning local struct SDL_Rect
	
	;; Member name: x Struct offset = 128. Member offset: 0
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 128], eax
	
	;; Member name: y Struct offset = 128. Member offset: 4
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 124], 100
	
	;; Member name: w Struct offset = 128. Member offset: 8
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 120], 50
	
	;; Member name: h Struct offset = 128. Member offset: 12
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 116], 50
	
	.loop_0:
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 1
	mov rax, [rbp - 80]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_1:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_1
	
	;; --- break ----
	jmp .loop_end_0
	
	jmp .if_end_1
	.if_end_1:
	
	call SDL_GetTicks
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 88], rax
	
	;; Minus get the two operands from the stack
	mov rbx, [rbp - 96]
	mov rax, [rbp - 88]
	sub rax, rbx
	
	mov rbx, [rbp - 104]
	mov rcx, rax
	add rbx, rcx
	mov [rbp - 104], rbx
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, [rbp - 72]
	mov rax, [rbp - 104]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovl rax, rbx
	
	.if_2:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_2
	
	;; --- continue ----
	jmp .loop_0_end_start
	
	jmp .if_end_2
	.if_end_2:
	
	mov rax, [rbp - 64]
	
	;; Moving argument number 1
	mov rdi, rax
	
	
	;; Moving argument number 2
	mov rsi, 0
	
	
	;; Moving argument number 3
	mov rdx, 0
	
	
	;; Moving argument number 4
	mov rcx, 0
	
	
	;; Moving argument number 5
	mov r8, 255
	
	call SDL_SetRenderDrawColor
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 112], rax
	
	mov rax, [rbp - 64]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call SDL_RenderClear
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 112], rax
	
	xor rax, rax
	mov rax, event
	
	;; Moving argument number 1
	mov rdi, rax
	
	call SDL_PollEvent
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 112], rax
	
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rbx, 0
	mov rax, [rbp - 112]
	cmp rax, rbx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmovne rax, rbx
	
	.if_3:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_3
	
	;; Dereferencing variable event. handle_global_ptr
	mov rbx, event
	mov rbx, [rbx]
	xor rax, rax
	mov eax, ebx
	;; Finish dereferencing variable event
	
	mov ebx, [SDL_QUIT]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rbx
	mov rcx, rax
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.if_4:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_4
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 80], 1
	
	jmp .elif_4_0_end
	.if_end_4:
	
	;; Dereferencing variable event. handle_global_ptr
	mov rbx, event
	mov rbx, [rbx]
	xor rax, rax
	mov eax, ebx
	;; Finish dereferencing variable event
	
	mov ebx, [SDL_KEYDOWN]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rbx
	mov rcx, rax
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.elif_4_0:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_4_0_end
	
	xor rax, rax
	mov rax, event
	
	
	
	mov rbx, rax
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	mov rax, 5
	mov rcx, 4
	mul rcx
	
	;; Plus get the two operands from the stack
	mov rcx, rax
	mov rdx, rbx
	add rcx, rdx
	;; will lock rcx. first = rax. second = rbx. Locked: [rbx, rax, rcx, rdx]
	
	;; assign_local_pointer of type Integer32
	mov rax, rcx
	mov [rbp - 136], rax
	
	;; Dereferencing variable keysym. handle_local_ptr_int_float
	mov rax, [rbp - 136]
	mov rax, [rax]
	xor rbx, rbx
	mov ebx, eax
	
	mov eax, [SDL_ESC]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rax
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	;; Dereferencing variable keysym. handle_local_ptr_int_float
	mov rbx, [rbp - 136]
	mov rbx, [rbx]
	xor rcx, rcx
	mov ecx, ebx
	
	mov ebx, [SDLK_q]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rsi, rbx
	mov rdx, rcx
	cmp rdx, rsi
	;; Not xor-ing here as it sets flags
	mov rbx, 0
	mov rcx, 1
	cmove rbx, rcx
	
	;; gen_logical_statement
	xor rcx, rcx
	mov rcx, rbx
	xor rdx, rdx
	mov rdx, rax
	or rcx, rdx
	
	.if_5:
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_5
	
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 80], 1
	
	jmp .elif_5_3_end
	.if_end_5:
	
	;; Dereferencing variable keysym. handle_local_ptr_int_float
	mov rax, [rbp - 136]
	mov rax, [rax]
	xor rbx, rbx
	mov ebx, eax
	
	mov eax, [SDLK_j]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rax
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.elif_5_0:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_5_0_end
	
	mov eax, [RECT_SPEED]
	
	;; Minus get the two operands from the stack
	mov rcx, rax
	mov rbx, [rbp - 124]
	sub rbx, rcx
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 124], ebx
	
	jmp .elif_5_0_end
	.elif_5_0_end:
	
	;; Dereferencing variable keysym. handle_local_ptr_int_float
	mov rax, [rbp - 136]
	mov rax, [rax]
	xor rbx, rbx
	mov ebx, eax
	
	mov eax, [SDLK_k]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rax
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.elif_5_1:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_5_1_end
	
	mov eax, [RECT_SPEED]
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rcx, [rbp - 124]
	add rbx, rcx
	;; will lock rbx. first = rax. second = [rbp - 124]. Locked: [rax, rbx, rcx]
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 124], ebx
	
	jmp .elif_5_1_end
	.elif_5_1_end:
	
	;; Dereferencing variable keysym. handle_local_ptr_int_float
	mov rax, [rbp - 136]
	mov rax, [rax]
	xor rbx, rbx
	mov ebx, eax
	
	mov eax, [SDLK_h]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rax
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.elif_5_2:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_5_2_end
	
	mov eax, [RECT_SPEED]
	
	;; Minus get the two operands from the stack
	mov rcx, rax
	mov rbx, [rbp - 128]
	sub rbx, rcx
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 128], ebx
	
	jmp .elif_5_2_end
	.elif_5_2_end:
	
	;; Dereferencing variable keysym. handle_local_ptr_int_float
	mov rax, [rbp - 136]
	mov rax, [rax]
	xor rbx, rbx
	mov ebx, eax
	
	mov eax, [SDLK_l]
	
	;; We pop in the opposite order of comparison as we push onto the stack
	mov rdx, rax
	mov rcx, rbx
	cmp rcx, rdx
	;; Not xor-ing here as it sets flags
	mov rax, 0
	mov rbx, 1
	cmove rax, rbx
	
	.elif_5_3:
	cmp rax, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_5_3_end
	
	mov eax, [RECT_SPEED]
	
	;; Plus get the two operands from the stack
	mov rbx, rax
	mov rcx, [rbp - 128]
	add rbx, rcx
	;; will lock rbx. first = rax. second = [rbp - 128]. Locked: [rax, rbx, rcx]
	
	;; assign_local_number of type Integer32
	mov DWORD [rbp - 128], ebx
	
	jmp .elif_5_3_end
	.elif_5_3_end:
	
	jmp .elif_4_0_end
	.elif_4_0_end:
	
	jmp .if_end_3
	.if_end_3:
	
	mov rax, [rbp - 64]
	
	;; Moving argument number 1
	mov rdi, rax
	
	
	;; Moving argument number 2
	mov rsi, 255
	
	
	;; Moving argument number 3
	mov rdx, 0
	
	
	;; Moving argument number 4
	mov rcx, 0
	
	
	;; Moving argument number 5
	mov r8, 255
	
	call SDL_SetRenderDrawColor
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 112], rax
	
	mov rax, [rbp - 64]
	
	;; Moving argument number 1
	mov rdi, rax
	
	;; Storing address of struct SDL_Rect for variable rect not in handle_local_ptr
	lea rax, [rbp - 128]
	
	;; Moving argument number 2
	mov rsi, rax
	
	call SDL_RenderFillRect
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 112], rax
	
	mov rax, [rbp - 64]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call SDL_RenderPresent
	
	call SDL_GetTicks
	
	;; assign_local_number of type Integer
	mov QWORD [rbp - 96], rax
	
	.loop_0_end_start:
	jmp .loop_0
	.loop_end_0:
	
	mov rax, [rbp - 64]
	
	;; Moving argument number 1
	mov rdi, rax
	
	call SDL_DestroyRenderer
	
	call SDL_Quit
	
	mov rsp, rbp
	pop rbp
	ret
	

