use core::arch::asm;

pub fn cpuid(leaf: u32) -> [u32; 4] {
    let mut x: [u32; 4] = [0; 4];

    unsafe {
        asm!("push rbx",
        "cpuid",
        "mov esi, ebx",
        "pop rbx", inout("eax") leaf => x[0], out("esi") x[1], out("edx") x[2], out("ecx") x[3]);
    }

    x
}
