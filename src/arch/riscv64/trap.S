.section .text
.global _trap_handler
.type _trap_handler, @function
_trap_handler:
    call   handle_trap
.size _trap_handler, . - _trap_handler
