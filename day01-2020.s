	.text
	.file	"repl"
	.globl	program
	.p2align	4, 0x90
	.type	program,@function
program:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movq	%rdi, %rbx
	addq	$-1, %rbx
	movq	%rbx, %rdi
	callq	part1@PLT
	movq	%rbx, %rdi
	callq	part2@PLT
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	program, .Lfunc_end0-program
	.cfi_endproc

	.globl	part1
	.p2align	4, 0x90
	.type	part1,@function
part1:
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r13
	.cfi_def_cfa_offset 32
	pushq	%r12
	.cfi_def_cfa_offset 40
	pushq	%rbx
	.cfi_def_cfa_offset 48
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r13, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%rdi, %r15
	movl	$1, %r14d
	movl	$2, %r13d
	cmpq	%r15, %r14
	jg	.LBB1_6
.LBB1_2:
	movq	%r14, %rdi
	callq	int_arg@PLT
	movq	%rax, %r12
	movq	%r13, %rbx
	.p2align	4, 0x90
.LBB1_3:
	cmpq	%r15, %rbx
	jg	.LBB1_7
	movq	%rbx, %rdi
	callq	int_arg@PLT
	leaq	(%rax,%r12), %rcx
	addq	$1, %rbx
	cmpq	$2020, %rcx
	jne	.LBB1_3
	jmp	.LBB1_5
	.p2align	4, 0x90
.LBB1_7:
	addq	$1, %r14
	addq	$1, %r13
	cmpq	%r15, %r14
	jle	.LBB1_2
	jmp	.LBB1_6
.LBB1_5:
	imulq	%r12, %rax
	movq	%rax, %rdi
	callq	print_int@PLT
.LBB1_6:
	popq	%rbx
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	part1, .Lfunc_end1-part1
	.cfi_endproc

	.globl	part2
	.p2align	4, 0x90
	.type	part2,@function
part2:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$24, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdi, %r15
	movl	$1, %r12d
	movl	$3, %r14d
	cmpq	%r15, %r12
	jg	.LBB2_9
.LBB2_2:
	movq	%r12, %rdi
	callq	int_arg@PLT
	movq	%rax, %r13
	movq	%r14, 8(%rsp)
	movq	%r12, 16(%rsp)
.LBB2_4:
	addq	$1, %r12
	cmpq	%r15, %r12
	jg	.LBB2_10
	movq	%r12, %rdi
	callq	int_arg@PLT
	movq	%rax, %rbx
	movq	%r14, %rbp
	.p2align	4, 0x90
.LBB2_6:
	cmpq	%r15, %rbp
	jg	.LBB2_3
	movq	%rbp, %rdi
	callq	int_arg@PLT
	leaq	(%rbx,%r13), %rcx
	addq	%rax, %rcx
	addq	$1, %rbp
	cmpq	$2020, %rcx
	jne	.LBB2_6
	jmp	.LBB2_8
	.p2align	4, 0x90
.LBB2_3:
	addq	$1, %r14
	jmp	.LBB2_4
.LBB2_10:
	movq	16(%rsp), %r12
	addq	$1, %r12
	movq	8(%rsp), %r14
	addq	$1, %r14
	cmpq	%r15, %r12
	jle	.LBB2_2
	jmp	.LBB2_9
.LBB2_8:
	imulq	%r13, %rbx
	imulq	%rax, %rbx
	movq	%rbx, %rdi
	callq	print_int@PLT
.LBB2_9:
	addq	$24, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	part2, .Lfunc_end2-part2
	.cfi_endproc

	.globl	analyze
	.p2align	4, 0x90
	.type	analyze,@function
analyze:
	.cfi_startproc
	retq
.Lfunc_end3:
	.size	analyze, .Lfunc_end3-analyze
	.cfi_endproc

	.section	".note.GNU-stack","",@progbits
