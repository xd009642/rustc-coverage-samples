
target/coverage-debug/stable/13_unwind_continuation/sample_13_unwind_continuation:	file format elf64-x86-64

Disassembly of section .text:

0000000000018c30 <_RNvCsaonflgrxSWB_29sample_13_unwind_continuation4work>:
; _RNvCsaonflgrxSWB_29sample_13_unwind_continuation4work():
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/13_unwind_continuation/src/main.rs:9
   18c30: 48 83 ec 58                  	subq	$0x58, %rsp
   18c34: 48 89 7c 24 08               	movq	%rdi, 0x8(%rsp)
   18c39: 48 89 7c 24 40               	movq	%rdi, 0x40(%rsp)
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/13_unwind_continuation/src/main.rs:10
   18c3e: f0                           	lock
   18c3f: 48 ff 05 e2 6d 04 00         	incq	0x46de2(%rip)           # 0x5fa28 <__start___llvm_prf_cnts+0x20>
   18c46: 48 8b 05 db 49 04 00         	movq	0x449db(%rip), %rax     # 0x5d628 <writev+0x5d628>
   18c4d: 48 8d 7c 24 20               	leaq	0x20(%rsp), %rdi
   18c52: 48 89 7c 24 10               	movq	%rdi, 0x10(%rsp)
   18c57: ff d0                        	callq	*%rax
   18c59: 48 8b 7c 24 10               	movq	0x10(%rsp), %rdi
   18c5e: e8 ed f4 ff ff               	callq	0x18150 <_RNvXsd_NtCs9k3SxhrAWiO_3std3envNtB5_4ArgsNtNtNtNtCsgxBkk5gSRhY_4core4iter6traits10exact_size17ExactSizeIterator3lenCsaonflgrxSWB_29sample_13_unwind_continuation>
   18c63: 48 89 44 24 18               	movq	%rax, 0x18(%rsp)
   18c68: eb 1c                        	jmp	0x18c86 <_RNvCsaonflgrxSWB_29sample_13_unwind_continuation4work+0x56>
   18c6a: 48 8d 7c 24 20               	leaq	0x20(%rsp), %rdi
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/13_unwind_continuation/src/main.rs:10
   18c6f: e8 8c f7 ff ff               	callq	0x18400 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueNtNtCs9k3SxhrAWiO_3std3env4ArgsECsaonflgrxSWB_29sample_13_unwind_continuation>
   18c74: eb 67                        	jmp	0x18cdd <_RNvCsaonflgrxSWB_29sample_13_unwind_continuation4work+0xad>
   18c76: 48 89 c1                     	movq	%rax, %rcx
   18c79: 89 d0                        	movl	%edx, %eax
   18c7b: 48 89 4c 24 48               	movq	%rcx, 0x48(%rsp)
   18c80: 89 44 24 50                  	movl	%eax, 0x50(%rsp)
   18c84: eb e4                        	jmp	0x18c6a <_RNvCsaonflgrxSWB_29sample_13_unwind_continuation4work+0x3a>
   18c86: 48 8b 44 24 18               	movq	0x18(%rsp), %rax
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/13_unwind_continuation/src/main.rs:10
   18c8b: 48 83 e8 01                  	subq	$0x1, %rax
   18c8f: 0f 94 c0                     	sete	%al
   18c92: 0f b6 f8                     	movzbl	%al, %edi
   18c95: e8 56 00 00 00               	callq	0x18cf0 <_RNvCsaonflgrxSWB_29sample_13_unwind_continuation9may_panic>
   18c9a: eb 00                        	jmp	0x18c9c <_RNvCsaonflgrxSWB_29sample_13_unwind_continuation4work+0x6c>
   18c9c: 48 8d 7c 24 20               	leaq	0x20(%rsp), %rdi
   18ca1: e8 5a f7 ff ff               	callq	0x18400 <_RINvNtCsgxBkk5gSRhY_4core3ptr9drop_glueNtNtCs9k3SxhrAWiO_3std3env4ArgsECsaonflgrxSWB_29sample_13_unwind_continuation>
   18ca6: 48 8b 44 24 08               	movq	0x8(%rsp), %rax
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/13_unwind_continuation/src/main.rs:11
   18cab: 8b 08                        	movl	(%rax), %ecx
   18cad: 89 c8                        	movl	%ecx, %eax
   18caf: 83 c0 01                     	addl	$0x1, %eax
   18cb2: 89 44 24 04                  	movl	%eax, 0x4(%rsp)
   18cb6: 39 c8                        	cmpl	%ecx, %eax
   18cb8: 72 10                        	jb	0x18cca <_RNvCsaonflgrxSWB_29sample_13_unwind_continuation4work+0x9a>
   18cba: 48 8b 44 24 08               	movq	0x8(%rsp), %rax
   18cbf: 8b 4c 24 04                  	movl	0x4(%rsp), %ecx
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/13_unwind_continuation/src/main.rs:11
   18cc3: 89 08                        	movl	%ecx, (%rax)
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/13_unwind_continuation/src/main.rs:12
   18cc5: 48 83 c4 58                  	addq	$0x58, %rsp
   18cc9: c3                           	retq
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/13_unwind_continuation/src/main.rs:11
   18cca: 48 8d 3d 87 2a 04 00         	leaq	0x42a87(%rip), %rdi     # 0x5b758 <__dso_handle+0xc8>
   18cd1: ff 15 59 49 04 00            	callq	*0x44959(%rip)          # 0x5d630 <writev+0x5d630>
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/13_unwind_continuation/src/main.rs:9
   18cd7: ff 15 03 49 04 00            	callq	*0x44903(%rip)          # 0x5d5e0 <writev+0x5d5e0>
   18cdd: 48 8b 7c 24 48               	movq	0x48(%rsp), %rdi
   18ce2: e8 d9 16 04 00               	callq	0x5a3c0 <_Unwind_Resume@plt>
   18ce7: cc                           	int3
   18ce8: cc                           	int3
   18ce9: cc                           	int3
   18cea: cc                           	int3
   18ceb: cc                           	int3
   18cec: cc                           	int3
   18ced: cc                           	int3
   18cee: cc                           	int3
   18cef: cc                           	int3
