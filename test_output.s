	.text
	.file	"test_ir_module"
	.globl	main_character                  # -- Begin function main_character
	.p2align	4, 0x90
	.type	main_character,@function
main_character:                         # @main_character
	.cfi_startproc
# %bb.0:                                # %entry
	retq
.Lfunc_end0:
	.size	main_character, .Lfunc_end0-main_character
	.cfi_endproc
                                        # -- End function
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	xorl	%eax, %eax
	retq
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc
                                        # -- End function
	.type	.Lvibez,@object                 # @vibez
	.local	.Lvibez
	.comm	.Lvibez,8,8
	.section	".note.GNU-stack","",@progbits
