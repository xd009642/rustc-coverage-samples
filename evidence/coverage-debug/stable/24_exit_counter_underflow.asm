
target/coverage-debug/stable/24_exit_counter_underflow/sample_24_exit_counter_underflow:	file format elf64-x86-64

Disassembly of section .text:

0000000000017ed0 <_RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow3run>:
; _RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow3run():
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:10
   17ed0: 48 83 ec 58                  	subq	$0x58, %rsp
   17ed4: 48 89 7c 24 08               	movq	%rdi, 0x8(%rsp)
   17ed9: 48 89 7c 24 40               	movq	%rdi, 0x40(%rsp)
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:12
   17ede: f0                           	lock
   17edf: 48 ff 05 aa 78 04 00         	incq	0x478aa(%rip)           # 0x5f790 <__start___llvm_prf_cnts>
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:11
   17ee6: c7 44 24 10 00 00 00 00      	movl	$0x0, 0x10(%rsp)
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:12
   17eee: f0                           	lock
   17eef: 48 ff 05 a2 78 04 00         	incq	0x478a2(%rip)           # 0x5f798 <__start___llvm_prf_cnts+0x8>
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:13
   17ef6: 8b 4c 24 10                  	movl	0x10(%rsp), %ecx
   17efa: 89 c8                        	movl	%ecx, %eax
   17efc: 83 c0 01                     	addl	$0x1, %eax
   17eff: 89 44 24 04                  	movl	%eax, 0x4(%rsp)
   17f03: 39 c8                        	cmpl	%ecx, %eax
   17f05: 72 2c                        	jb	0x17f33 <_RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow3run+0x63>
   17f07: 48 8b 74 24 08               	movq	0x8(%rsp), %rsi
   17f0c: 8b 44 24 04                  	movl	0x4(%rsp), %eax
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:13
   17f10: 89 44 24 10                  	movl	%eax, 0x10(%rsp)
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:14
   17f14: 8b 7c 24 10                  	movl	0x10(%rsp), %edi
   17f18: e8 23 01 00 00               	callq	0x18040 <_RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow4step>
   17f1d: 89 44 24 18                  	movl	%eax, 0x18(%rsp)
   17f21: 89 54 24 1c                  	movl	%edx, 0x1c(%rsp)
   17f25: 8b 44 24 18                  	movl	0x18(%rsp), %eax
   17f29: 48 a9 01 00 00 00            	testq	$0x1, %rax
   17f2f: 75 0f                        	jne	0x17f40 <_RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow3run+0x70>
   17f31: eb 2e                        	jmp	0x17f61 <_RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow3run+0x91>
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:13
   17f33: 48 8d 3d 8e 34 04 00         	leaq	0x4348e(%rip), %rdi     # 0x5b3c8 <__dso_handle+0x78>
   17f3a: ff 15 20 54 04 00            	callq	*0x45420(%rip)          # 0x5d360 <writev+0x5d360>
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:18
   17f40: f0                           	lock
   17f41: 48 ff 05 58 78 04 00         	incq	0x47858(%rip)           # 0x5f7a0 <__start___llvm_prf_cnts+0x10>
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:16
   17f48: 8b 44 24 1c                  	movl	0x1c(%rsp), %eax
   17f4c: 89 44 24 4c                  	movl	%eax, 0x4c(%rsp)
   17f50: 89 44 24 14                  	movl	%eax, 0x14(%rsp)
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:18
   17f54: 8b 44 24 14                  	movl	0x14(%rsp), %eax
   17f58: ff c8                        	decl	%eax
   17f5a: 83 e8 02                     	subl	$0x2, %eax
   17f5d: 72 12                        	jb	0x17f71 <_RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow3run+0xa1>
   17f5f: eb 02                        	jmp	0x17f63 <_RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow3run+0x93>
   17f61: eb 8b                        	jmp	0x17eee <_RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow3run+0x1e>
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:22
   17f63: 8b 54 24 14                  	movl	0x14(%rsp), %edx
   17f67: b8 01 00 00 00               	movl	$0x1, %eax
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:25
   17f6c: 48 83 c4 58                  	addq	$0x58, %rsp
   17f70: c3                           	retq
; /home/xd009642/personal/tarpaulin/rustc-coverage-samples/failing/24_exit_counter_underflow/src/main.rs:20
   17f71: 48 8d 44 24 14               	leaq	0x14(%rsp), %rax
   17f76: 48 89 44 24 50               	movq	%rax, 0x50(%rsp)
   17f7b: 48 8d 7c 24 30               	leaq	0x30(%rsp), %rdi
   17f80: 48 8d 74 24 14               	leaq	0x14(%rsp), %rsi
   17f85: e8 76 fd ff ff               	callq	0x17d00 <_RINvMNtNtCsgxBkk5gSRhY_4core3fmt2rtNtB3_8Argument11new_displaymECsb8Y68SjZ9S4_32sample_24_exit_counter_underflow>
   17f8a: 48 8b 44 24 30               	movq	0x30(%rsp), %rax
   17f8f: 48 89 44 24 20               	movq	%rax, 0x20(%rsp)
   17f94: 48 8b 44 24 38               	movq	0x38(%rsp), %rax
   17f99: 48 89 44 24 28               	movq	%rax, 0x28(%rsp)
   17f9e: 48 8d 3d 23 11 ff ff         	leaq	-0xeedd(%rip), %rdi     # 0x90c8 <writev+0x90c8>
   17fa5: 48 8d 74 24 20               	leaq	0x20(%rsp), %rsi
   17faa: e8 51 fc ff ff               	callq	0x17c00 <_RINvMs2_NtCsgxBkk5gSRhY_4core3fmtNtB6_9Arguments3newKj4_Kj1_ECsb8Y68SjZ9S4_32sample_24_exit_counter_underflow>
   17faf: 48 89 c7                     	movq	%rax, %rdi
   17fb2: 48 89 d6                     	movq	%rdx, %rsi
   17fb5: ff 15 ad 53 04 00            	callq	*0x453ad(%rip)          # 0x5d368 <writev+0x5d368>
   17fbb: e9 2e ff ff ff               	jmp	0x17eee <_RNvCsb8Y68SjZ9S4_32sample_24_exit_counter_underflow3run+0x1e>
