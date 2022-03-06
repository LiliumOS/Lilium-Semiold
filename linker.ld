ENTRY(_start)

PHDRS
{
    headers PT_PHDR PHDRS;
    text PT_LOAD FILEHDR PHDRS;
    data PT_LOAD;
    dynamic PT_DYNAMIC;
}

SECTIONS
{
    kernel_phys_offset = 0xffffffff80000000;
    . = kernel_phys_offset + 0x100000;

    .stivale2hdr ALIGN(4K) :
    {
        KEEP(*(.stivale2hdr))
    } :text

    .text ALIGN(4K) :
    {
        KEEP(*(.text*))
    }

    .rodata ALIGN(4K) :
    {
        KEEP(*(.rodata*))
    }

    .data ALIGN(4K) :
    {
        KEEP(*(.data*))
    } :data

    .bss ALIGN(4K) :
    {
        KEEP(*(COMMON))
        KEEP(*(.bss*))
    }

    .got ALIGN(4K) :
    {
        _GLOBAL_OFFSET_TABLE_ = .;
        KEEP(*(.got*))
    }

    .dynamic ALIGN(4K) : {
        _DYNAMIC = .;
        KEEP(*(.dynamic*));
    }
}
