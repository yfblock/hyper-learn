.global _start
.section .text.entry
_start:
    # Load the address of the stack pointer
    la      sp, __stack_top
    
    # Jump to the Rust main function
    call    rust_main

# If we return from rust_main, we can loop indefinitely
.L0:
    wfi
    j       .L0
