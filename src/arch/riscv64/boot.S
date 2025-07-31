.global _start
.section .text.entry
.type _start, @function
_start:
    # Load the address of the stack pointer
    la      sp, __stack_top

    call    _clear_bss
    
    # Setup the trap handler
    la      s0, _trap_handler
    csrw    stvec, s0
    
    # Jump to the Rust main function
    call    rust_main

# If we return from rust_main, we can loop indefinitely
.L0:
    wfi
    j       .L0
.size _start, . - _start

# Clear the .bss section
.type _clear_bss, @function
_clear_bss:
    la      t0, _sbss
    la      t1, _ebss
1:
    sb      zero, 0(t0)
    addi    t0, t0, 1
    blt     t0, t1, 1b
    ret
.size _clear_bss, . - _clear_bss
