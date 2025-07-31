.section .bss.bstack
.global __stack_top
.p2align 3
__stack_start:
    .space 40960
__stack_top:
