.global _start
.section .text.entry
.type _start, @function
_start:
    # Load the address of the stack pointer
    la      sp, __stack_top
    
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
