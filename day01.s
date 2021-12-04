	.text
	.file	"repl"
	.globl	program
	.p2align	4, 0x90
	.type	program,@function
program:
	.cfi_startproc
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
	movq	%rdi, %r14
	addq	$-1, %r14
	movq	%r14, %rdi
	callq	print_int@PLT
	movl	$1, %edi
	callq	int_arg@PLT
	movq	%rax, %r12
	movl	$2, %ebx
	xorl	%r15d, %r15d
	jmp	.LBB0_1
	.p2align	4, 0x90
.LBB0_4:
	addq	$1, %rbx
	movq	%rax, %r12
.LBB0_1:
	cmpq	%r14, %rbx
	jg	.LBB0_5
	movq	%rbx, %rdi
	callq	int_arg@PLT
	cmpq	%r12, %rax
	jle	.LBB0_4
	addq	$1, %r15
	jmp	.LBB0_4
.LBB0_5:
	movq	%r15, %rdi
	callq	print_int@PLT
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
	.size	program, .Lfunc_end0-program
	.cfi_endproc

	.globl	analyze
	.p2align	4, 0x90
	.type	analyze,@function
analyze:
	.cfi_startproc
	retq
.Lfunc_end1:
	.size	analyze, .Lfunc_end1-analyze
	.cfi_endproc

	.section	".note.GNU-stack","",@progbits
