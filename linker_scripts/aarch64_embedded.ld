/* ARM64 Embedded Linker Script for CURSED Compiler */

MEMORY
{
    FLASH (rx) : ORIGIN = 0x8000000, LENGTH = 2M
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 256K
}

ENTRY(_start)

SECTIONS
{
    .text : {
        KEEP(*(.vector_table))
        *(.text*)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    .data : {
        _data_start = .;
        *(.data*)
        . = ALIGN(4);
        _data_end = .;
    } > RAM AT > FLASH

    _data_load_start = LOADADDR(.data);

    .bss : {
        _bss_start = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        _bss_end = .;
    } > RAM

    .heap : {
        _heap_start = .;
        . = . + 32K;
        _heap_end = .;
    } > RAM

    .stack : {
        . = . + 8K;
        _stack_top = .;
    } > RAM

    /DISCARD/ : {
        *(.ARM.exidx*)
        *(.ARM.extab*)
    }
}