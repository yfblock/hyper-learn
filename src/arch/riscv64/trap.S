.macro LDR  reg, offset
    ld  \reg, \offset*8(sp)
.endm

.macro STR  reg, offset
    sd  \reg, \offset*8(sp)
.endm

.macro LDCSR csr, tmp, offset
    csrr \tmp, \csr
    STR \tmp, \offset
.endm

.macro STCSR csr, tmp, offset
    LDR \tmp, \offset
    csrw \csr, \tmp
.endm

.macro SAVE_GENERAL_REGS
    STR     x1, 1
    csrr    x1, sscratch
    STR     x1, 2
    STR     x3, 3
    STR     x4, 4
    STR     x5, 5
    STR     x6, 6
    STR     x7, 7
    STR     x8, 8
    STR     x9, 9
    STR     x10, 10
    STR     x11, 11
    STR     x12, 12
    STR     x13, 13
    STR     x14, 14
    STR     x15, 15
    STR     x16, 16
    STR     x17, 17
    STR     x18, 18
    STR     x19, 19
    STR     x20, 20
    STR     x21, 21
    STR     x22, 22
    STR     x23, 23
    STR     x24, 24
    STR     x25, 25
    STR     x26, 26
    STR     x27, 27
    STR     x28, 28
    STR     x29, 29
    STR     x30, 30
    STR     x31, 31
.endm

.macro LOAD_GENERAL_REGS
    LDR     x1, 1
    LDR     x3, 3
    LDR     x4, 4
    LDR     x5, 5
    LDR     x6, 6
    LDR     x7, 7
    LDR     x8, 8
    LDR     x9, 9
    LDR     x10, 10
    LDR     x11, 11
    LDR     x12, 12
    LDR     x13, 13
    LDR     x14, 14
    LDR     x15, 15
    LDR     x16, 16
    LDR     x17, 17
    LDR     x18, 18
    LDR     x19, 19
    LDR     x20, 20
    LDR     x21, 21
    LDR     x22, 22
    LDR     x23, 23
    LDR     x24, 24
    LDR     x25, 25
    LDR     x26, 26
    LDR     x27, 27
    LDR     x28, 28
    LDR     x29, 29
    LDR     x30, 30
    LDR     x31, 31

    LDR     x2, 2
.endm

.section .text
.global _trap_handler
.type _trap_handler, @function
.p2align 2
_trap_handler:
    csrrw   sp, sscratch, sp        # swap(sscratch, sp)
    bne     sp, zero, .Luser        # if sp != 0 then goto .Luser
    csrr    sp, sscratch            # else sp = sscratch

.Luser:
    addi    sp, sp, -{context_size}
    SAVE_GENERAL_REGS

    LDCSR   sstatus, t0, 32
    LDCSR   sepc,    t0, 33
    LDCSR   hstatus, t0, 36

    mv      a0, sp

    call    handle_trap
.size _trap_handler, . - _trap_handler

.global vmenter
.type vmenter, @function
vmenter:
    STCSR   hstatus, t0, 36
    STCSR   sstatus, t0, 32
    STCSR   sepc,    t0, 33

    LOAD_GENERAL_REGS

    sret
.size vmenter, . - vmenter
