qemu-system-x86_64 -cdrom phantomos.iso -s -S > /dev/null 2>&1 &
gdb -ex 'file build-iso/phantomos.elf' -ex 'break main' -ex 'set disassembly-flavor intel' -ex 'target remote :1234' -ex 'layout split' -ex 'continue'
