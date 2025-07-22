#[macro_use]
mod macros;

use tock_registers::register_bitfields;

register_bitfields![
    u32,
    pub hedeleg [
        INST_ADDR_MISALIGN 0,
        INST_ACCESSS_FAULT 1,
        ILLEGAL_INST 2,
        BREAKPOINT 3,
        LOAD_ADDR_MISALIGNED 4,
        LOAD_ACCESS_FAULT 5,
        STORE_ADDR_MISALIGNED 6,
        STORE_ACCESS_FAULT 7,
        ENV_CALL_FROM_U_OR_VU 8,
        ENV_CALL_FROM_HS 9,
        ENV_CALL_FROM_VS 10,
        ENV_CALL_FROM_M 11,
        INST_PAGE_FAULT 12,
        LOAD_PAGE_FAULT 13,
        STORE_PAGE_FAULT 15,
        INST_GUEST_PAGE_FAULT 20,
        LOAD_GUEST_PAGE_FAULT 21,
        VIRTUAL_INST 22,
        STORE_GUEST_PAGE_FAULT 23,
    ],
    pub hideleg [
        VSSIP 2,
        VSTIP 6,
        VSEIP 10,
    ]
];
