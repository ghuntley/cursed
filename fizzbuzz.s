	.text
	.file	"fizzbuzz.ll"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:                                # %entry
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r12
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	pushq	%rax
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -40
	.cfi_offset %r12, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movl	$1, %ebx
	movl	$3435973837, %r14d              # imm = 0xCCCCCCCD
	movl	$2863311531, %r15d              # imm = 0xAAAAAAAB
	movl	$2290649225, %r12d              # imm = 0x88888889
	cmpl	$100, %ebx
	jle	.LBB0_2
	jmp	.LBB0_10
	.p2align	4, 0x90
.LBB0_3:                                # %if.then
                                        #   in Loop: Header=BB0_2 Depth=1
	movl	$.L.str, %edi
.LBB0_4:                                # %for.cond
                                        #   in Loop: Header=BB0_2 Depth=1
	xorl	%eax, %eax
	callq	printf@PLT
	incl	%ebx
	cmpl	$100, %ebx
	jg	.LBB0_10
.LBB0_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	movl	%ebx, %eax
	movq	%rax, %rcx
	imulq	%r12, %rcx
	shrq	$35, %rcx
	leal	(%rcx,%rcx,4), %ecx
	leal	(%rcx,%rcx,2), %ecx
	movl	$1, %edx
	subl	%ecx, %edx
	addl	%ebx, %edx
	cmpl	$1, %edx
	je	.LBB0_3
# %bb.5:                                # %if.else
                                        #   in Loop: Header=BB0_2 Depth=1
	movq	%rax, %rcx
	imulq	%r15, %rcx
	shrq	$33, %rcx
	leal	(%rcx,%rcx,2), %ecx
	movl	$1, %edx
	subl	%ecx, %edx
	addl	%ebx, %edx
	cmpl	$1, %edx
	jne	.LBB0_7
# %bb.6:                                # %if.then4
                                        #   in Loop: Header=BB0_2 Depth=1
	movl	$.L.str.1, %edi
	jmp	.LBB0_4
	.p2align	4, 0x90
.LBB0_7:                                # %if.else6
                                        #   in Loop: Header=BB0_2 Depth=1
	imulq	%r14, %rax
	shrq	$34, %rax
	leal	(%rax,%rax,4), %eax
	movl	$1, %ecx
	subl	%eax, %ecx
	addl	%ebx, %ecx
	cmpl	$1, %ecx
	jne	.LBB0_9
# %bb.8:                                # %if.then9
                                        #   in Loop: Header=BB0_2 Depth=1
	movl	$.L.str.2, %edi
	jmp	.LBB0_4
.LBB0_9:                                # %if.else11
                                        #   in Loop: Header=BB0_2 Depth=1
	movl	$.L.str.3, %edi
	movl	%ebx, %esi
	xorl	%eax, %eax
	callq	printf@PLT
	incl	%ebx
	cmpl	$100, %ebx
	jle	.LBB0_2
.LBB0_10:                               # %for.end
	xorl	%eax, %eax
	addq	$8, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.type	.L.str,@object                  # @.str
	.section	.rodata.str1.1,"aMS",@progbits,1
.L.str:
	.asciz	"FizzBuzz\n"
	.size	.L.str, 10

	.type	.L.str.1,@object                # @.str.1
.L.str.1:
	.asciz	"Fizz\n"
	.size	.L.str.1, 6

	.type	.L.str.2,@object                # @.str.2
.L.str.2:
	.asciz	"Buzz\n"
	.size	.L.str.2, 6

	.type	.L.str.3,@object                # @.str.3
.L.str.3:
	.asciz	"%d\n"
	.size	.L.str.3, 4

	.section	".note.GNU-stack","",@progbits
