%include "std.asm"

section .bss
	digitSpace resb 100
	digitSpacePos resb 8
	argc resb 8
	current resb 800
	next resb 800

section .data
	string_0 db 32
	string_1 db 42
	string_2 db 10

section .text
	global _start

_start:
	mov [argc], rsp
	call _main
	
	exit 0

_main:
	push rbp
	mov rbp, rsp
	sub rsp, 192
	
	push 100
	
	;; for int float
	pop rax
	mov [rbp - 8], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	mov rax, current
	push rax
	
	mov rax, [rbp - 8]
	push rax
	
	push 8
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	push rax
	
	push 8
	
	push 3
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	push rax
	
	;; Minus get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 16], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	mov rax, current
	push rax
	
	mov rax, [rbp - 8]
	push rax
	
	push 8
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	push rax
	
	push 8
	
	push 2
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	push rax
	
	;; Minus get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 24], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	mov rax, current
	push rax
	
	mov rax, [rbp - 8]
	push rax
	
	push 8
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	push rax
	
	push 8
	
	;; Minus get the two operands from the stack
	pop rbx
	pop rax
	sub rax, rbx
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 32], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	push 1
	
	pop rax
	mov rbx, [rbp - 16]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	push 1
	
	pop rax
	mov rbx, [rbp - 24]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	push 0
	
	pop rax
	mov rbx, [rbp - 32]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	mov rax, current
	push rax
	
	pop rax
	mov [rbp - 40], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	mov rax, current
	push rax
	
	pop rax
	mov [rbp - 48], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	mov rax, current
	push rax
	
	pop rax
	mov [rbp - 56], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	push 0
	
	;; for int float
	pop rax
	mov [rbp - 64], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	push 0
	
	mov rax, [rbp - 8]
	push rax
	
	push 1
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 88], rcx
	mov [rbp - 80], rbx
	mov [rbp - 72], rax
	.loop_0:
	mov rcx, [rbp - 88]
	mov rbx, [rbp - 80]
	mov rax, [rbp - 72]
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_0
	inc rax
	inc rbx
	mov [rbp - 80], rbx
	mov [rbp - 72], rax
	
	push 0
	
	;; for int float
	pop rax
	mov [rbp - 64], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	push 0
	
	mov rax, [rbp - 8]
	push rax
	
	push 1
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	push 1
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 112], rcx
	mov [rbp - 104], rbx
	mov [rbp - 96], rax
	.loop_1:
	mov rcx, [rbp - 112]
	mov rbx, [rbp - 104]
	mov rax, [rbp - 96]
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_1
	inc rax
	inc rbx
	mov [rbp - 104], rbx
	mov [rbp - 96], rax
	
	mov rax, current
	push rax
	
	mov rax, [rbp - 64]
	push rax
	
	push 8
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 40], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	mov rax, current
	push rax
	
	mov rax, [rbp - 64]
	push rax
	
	push 8
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	push 8
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 48], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	mov rax, current
	push rax
	
	mov rax, [rbp - 64]
	push rax
	
	push 8
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	push 16
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 56], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	mov rax, next
	push rax
	
	mov rax, [rbp - 64]
	push rax
	
	push 8
	
	;; Multiply get the two operands from the stack
	xor rdx, rdx
	pop rax
	pop rbx
	mul rbx
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	push 8
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 120], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	;; Dereferencing variable first
	mov rax, [rbp - 40]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable first
	
	push 0
	
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
	
	;; Dereferencing variable second
	mov rax, [rbp - 48]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable second
	
	push 0
	
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
	
	;; Dereferencing variable third
	mov rax, [rbp - 56]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable third
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_2
	mov rax, 0
	jmp .skip_next2
	.skip_2:
	mov rax, 1
	.skip_next2:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	.if_0:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .if_end_0
	
	push 0
	
	pop rax
	mov rbx, [rbp - 120]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	jmp .elif_0_6_end
	.if_end_0:
	
	;; Dereferencing variable first
	mov rax, [rbp - 40]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable first
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_3
	mov rax, 0
	jmp .skip_next3
	.skip_3:
	mov rax, 1
	.skip_next3:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable second
	mov rax, [rbp - 48]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable second
	
	push 0
	
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
	
	;; Dereferencing variable third
	mov rax, [rbp - 56]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable third
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_5
	mov rax, 0
	jmp .skip_next5
	.skip_5:
	mov rax, 1
	.skip_next5:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	.elif_0_0:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_0_end
	
	push 1
	
	pop rax
	mov rbx, [rbp - 120]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	jmp .elif_0_0_end
	.elif_0_0_end:
	
	;; Dereferencing variable first
	mov rax, [rbp - 40]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable first
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_6
	mov rax, 0
	jmp .skip_next6
	.skip_6:
	mov rax, 1
	.skip_next6:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable second
	mov rax, [rbp - 48]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable second
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_7
	mov rax, 0
	jmp .skip_next7
	.skip_7:
	mov rax, 1
	.skip_next7:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable third
	mov rax, [rbp - 56]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable third
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_8
	mov rax, 0
	jmp .skip_next8
	.skip_8:
	mov rax, 1
	.skip_next8:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	.elif_0_1:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_1_end
	
	push 1
	
	pop rax
	mov rbx, [rbp - 120]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	jmp .elif_0_1_end
	.elif_0_1_end:
	
	;; Dereferencing variable first
	mov rax, [rbp - 40]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable first
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_9
	mov rax, 0
	jmp .skip_next9
	.skip_9:
	mov rax, 1
	.skip_next9:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable second
	mov rax, [rbp - 48]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable second
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_10
	mov rax, 0
	jmp .skip_next10
	.skip_10:
	mov rax, 1
	.skip_next10:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable third
	mov rax, [rbp - 56]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable third
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_11
	mov rax, 0
	jmp .skip_next11
	.skip_11:
	mov rax, 1
	.skip_next11:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	.elif_0_2:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_2_end
	
	push 1
	
	pop rax
	mov rbx, [rbp - 120]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	jmp .elif_0_2_end
	.elif_0_2_end:
	
	;; Dereferencing variable first
	mov rax, [rbp - 40]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable first
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_12
	mov rax, 0
	jmp .skip_next12
	.skip_12:
	mov rax, 1
	.skip_next12:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable second
	mov rax, [rbp - 48]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable second
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_13
	mov rax, 0
	jmp .skip_next13
	.skip_13:
	mov rax, 1
	.skip_next13:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable third
	mov rax, [rbp - 56]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable third
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_14
	mov rax, 0
	jmp .skip_next14
	.skip_14:
	mov rax, 1
	.skip_next14:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	.elif_0_3:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_3_end
	
	push 0
	
	pop rax
	mov rbx, [rbp - 120]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	jmp .elif_0_3_end
	.elif_0_3_end:
	
	;; Dereferencing variable first
	mov rax, [rbp - 40]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable first
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_15
	mov rax, 0
	jmp .skip_next15
	.skip_15:
	mov rax, 1
	.skip_next15:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable second
	mov rax, [rbp - 48]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable second
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_16
	mov rax, 0
	jmp .skip_next16
	.skip_16:
	mov rax, 1
	.skip_next16:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable third
	mov rax, [rbp - 56]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable third
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_17
	mov rax, 0
	jmp .skip_next17
	.skip_17:
	mov rax, 1
	.skip_next17:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	.elif_0_4:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_4_end
	
	push 1
	
	pop rax
	mov rbx, [rbp - 120]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	jmp .elif_0_4_end
	.elif_0_4_end:
	
	;; Dereferencing variable first
	mov rax, [rbp - 40]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable first
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_18
	mov rax, 0
	jmp .skip_next18
	.skip_18:
	mov rax, 1
	.skip_next18:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable second
	mov rax, [rbp - 48]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable second
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_19
	mov rax, 0
	jmp .skip_next19
	.skip_19:
	mov rax, 1
	.skip_next19:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable third
	mov rax, [rbp - 56]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable third
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_20
	mov rax, 0
	jmp .skip_next20
	.skip_20:
	mov rax, 1
	.skip_next20:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	.elif_0_5:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_5_end
	
	push 1
	
	pop rax
	mov rbx, [rbp - 120]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	jmp .elif_0_5_end
	.elif_0_5_end:
	
	;; Dereferencing variable first
	mov rax, [rbp - 40]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable first
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_21
	mov rax, 0
	jmp .skip_next21
	.skip_21:
	mov rax, 1
	.skip_next21:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable second
	mov rax, [rbp - 48]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable second
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_22
	mov rax, 0
	jmp .skip_next22
	.skip_22:
	mov rax, 1
	.skip_next22:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	;; Dereferencing variable third
	mov rax, [rbp - 56]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable third
	
	push 1
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_23
	mov rax, 0
	jmp .skip_next23
	.skip_23:
	mov rax, 1
	.skip_next23:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	pop rax
	pop rbx
	and rax, rbx
	push rax
	
	.elif_0_6:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .elif_0_6_end
	
	push 0
	
	pop rax
	mov rbx, [rbp - 120]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	jmp .elif_0_6_end
	.elif_0_6_end:
	
	mov rax, [rbp - 64]
	push rax
	
	push 1
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; for int float
	pop rax
	mov [rbp - 64], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	jmp .loop_1
	.loop_end_1:
	
	push 0
	
	;; for int float
	pop rax
	mov [rbp - 128], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	push 0
	
	mov rax, [rbp - 8]
	push rax
	
	push 1
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 152], rcx
	mov [rbp - 144], rbx
	mov [rbp - 136], rax
	.loop_2:
	mov rcx, [rbp - 152]
	mov rbx, [rbp - 144]
	mov rax, [rbp - 136]
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_2
	inc rax
	inc rbx
	mov [rbp - 144], rbx
	mov [rbp - 136], rax
	
	mov rax, next
	push rax
	
	mov rax, [rbp - 128]
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	mov rax, [rax]
	push rax
	
	push 0
	
	;; We pop in the opposite order of comparison as we push onto the stack
	pop rbx
	pop rax
	cmp rax, rbx
	je .skip_24
	mov rax, 0
	jmp .skip_next24
	.skip_24:
	mov rax, 1
	.skip_next24:
	;; push onto the stack whatever's in rax so rest of the program can use it
	push rax
	
	.if_1:
	pop rcx
	cmp rcx, 0
	;; if the comparison value is false, jump to the next label altogether
	je .else_1
	
	mov rax, string_0
	push rax
	push 1
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	jmp .else_end_1
	.if_end_1:
	
	.else_1:
	
	mov rax, string_1
	push rax
	push 1
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	.else_end_1:
	
	mov rax, [rbp - 128]
	push rax
	
	push 8
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; for int float
	pop rax
	mov [rbp - 128], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	jmp .loop_2
	.loop_end_2:
	
	mov rax, string_2
	push rax
	push 1
	
	;; Assuming length is pushed last
	pop r8
	;; Assuming string address is pushed first
	pop r9
	mov rax, 1
	mov rdi, 1
	mov rsi, r9
	mov rdx, r8
	syscall
	
	push 0
	
	;; for int float
	pop rax
	mov [rbp - 128], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	push 0
	
	mov rax, [rbp - 8]
	push rax
	
	push 1
	
	pop rcx
	pop rbx
	pop rax
	mov [rbp - 176], rcx
	mov [rbp - 168], rbx
	mov [rbp - 160], rax
	.loop_3:
	mov rcx, [rbp - 176]
	mov rbx, [rbp - 168]
	mov rax, [rbp - 160]
	add rax, rcx
	dec rax
	dec rbx
	cmp rax, rbx
	jg .loop_end_3
	inc rax
	inc rbx
	mov [rbp - 168], rbx
	mov [rbp - 160], rax
	
	mov rax, current
	push rax
	
	mov rax, [rbp - 128]
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 184], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	mov rax, next
	push rax
	
	mov rax, [rbp - 128]
	push rax
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	pop rax
	mov [rbp - 192], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	;; Dereferencing variable idx_into_next
	mov rax, [rbp - 192]
	mov rax, [rax]
	push rax
	;; Finish dereferencing variable idx_into_next
	
	pop rax
	mov rbx, [rbp - 184]
	mov [rbx], rax
	;; is_ptr_deref: true, is_array: false, is_string: false
	
	mov rax, [rbp - 128]
	push rax
	
	push 8
	
	;; Plus get the two operands from the stack
	pop rax
	pop rbx
	add rax, rbx
	push rax
	
	;; for int float
	pop rax
	mov [rbp - 128], rax
	;; is_ptr_deref: false, is_array: false, is_string: false
	
	jmp .loop_3
	.loop_end_3:
	
	jmp .loop_0
	.loop_end_0:
	
	mov rsp, rbp
	pop rbp
	ret
	

