OUTPUT_ARCH(%ARCH%)
ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    _skernel = .;

    .text ALIGN(4K): {
        *(.multiboot .multiboot.*)
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        etext = .;
    }

    .rodata ALIGN(4K): {
        srodata = .;
        *(.rodata .rodata.*)
        erodata = .;
    }

    .got : {
        __got_start = .;
        *(.got .got.*)
        __got_end = .;
    }

    .data ALIGN(4K): {
        _sdata = .;
        . = ALIGN(4K);
        *(.data.boot_page_table)

        *(.data .data.*)
        *(.sdata .sdata.*)
        _edata = .;
    }

    .bss ALIGN(4K): {
        _load_end = .;
        _sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        _ebss = .;
    }

    PROVIDE(_end = .);
    /DISCARD/ : {
        *(.comment) *(.gnu*) *(.note*) *(.eh_frame*)
    }
}