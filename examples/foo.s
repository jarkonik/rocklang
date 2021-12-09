	.text
	.file	"main"
	.globl	__main__                        # -- Begin function __main__
	.p2align	4, 0x90
	.type	__main__,@function
__main__:                               # @__main__
	.cfi_startproc
# %bb.0:                                # %entry
	movabsq	$4607182418800017408, %rax      # imm = 0x3FF0000000000000
	movq	%rax, -8(%rsp)
	retq
.Lfunc_end0:
	.size	__main__, .Lfunc_end0-__main__
	.cfi_endproc
                                        # -- End function
	.section	".note.GNU-stack","",@progbits
