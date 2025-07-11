; CURSED Assembly Output with Advanced Optimization
; Optimization Level: Default
; PGO Enabled: false
; LTO Enabled: true
; Size Optimization: false

.section .text
.globl main
.type main, @function
main:
	mov rdi, hello_str
	call puts
	mov eax, 0
	ret

.section .rodata
hello_str: .string "Hello from CURSED with advanced optimization!"

.section .note.GNU-stack,"",@progbits
