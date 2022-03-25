use core::arch::asm;
use std::{collection::HashSet, str::StringView, sync::OnceCell};

use crate::x86_64::features::X86Feature;

pub fn cpuid(leaf: u32, ecx: u32) -> [u32; 4] {
    let mut x: [u32; 4] = [0; 4];

    unsafe {
        asm!("push rbx",
        "cpuid",
        "mov esi, ebx",
        "pop rbx", inout("eax") leaf => x[0], out("esi") x[1], out("edx") x[2], inout("ecx") ecx => x[3]);
    }

    x
}

pub mod features {
    use std::collection::HashSet;

    #[derive(Debug)]
    pub struct X86FeatureFromStrError;

    impl core::fmt::Display for X86FeatureFromStrError {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.write_str("Unknown x86 feature")
        }
    }

    macro_rules! define_x86_features{
        {
            $(($enum:ident, $feature:literal)),* $(,)?
        } => {
            #[derive(Copy,Clone,Debug,Hash,PartialEq,Eq)]
            #[non_exhaustive]
            #[repr(i32)]
            pub enum X86Feature{
                $($enum,)*
            }

            impl X86Feature{
                pub fn feature_name(&self) -> &'static str{
                    match self{
                        $(#[allow(unreachable_patterns)] Self::$enum => $feature,)*
                    }
                }
            }

            impl core::str::FromStr for X86Feature{
                type Err = X86FeatureFromStrError;
                fn from_str(x: &str) -> Result<Self,Self::Err>{
                    match x{

                        $(#[allow(unreachable_patterns)] $feature => Ok(X86Feature::$enum),)*
                        _ => Err(X86FeatureFromStrError)
                    }
                }
            }

            impl core::fmt::Display for X86Feature{
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result{
                    match self{
                        $(Self::$enum => f.write_str($feature),)*
                    }
                }
            }
        }
    }

    define_x86_features! {
        (Sce, "sce"),
        (Mmx, "mmx"),
        (Sse, "sse"),
        (Sse2, "sse2"),
        (Sse3, "sse3"),
        (Ssse3, "ssse3"),
        (Sse4, "sse4"),
        (Sse4_1,"sse4.1"),
        (Sse4_2, "sse4.2"),
        (Avx, "avx"),
        (Avx2,"avx2"),
        (Avx512f,"avx512f"),
        (Avx512pf,"avx512pf"),
        (Avx512er, "avx512er"),
        (Avx512cd, "avx512cd"),
        (Avx512vl, "avx512vl"),
        (Avx512bw, "avx512bw"),
        (Avx512dq,"avx512dq"),
        (Avx512ifma, "avx512ifma"),
        (Avx512vbmi, "avx512vbmi"),
        (Sha, "sha"),
        (Aes, "aes"),
        (Pclmul, "pclmul"),
        (ClFlushOpt, "clflushopt"),
        (Clwb, "clwb"),
        (FsGsBase, "fsgsbase"),
        (Ptwrite, "ptwrite"),
        (Rdrand, "rdrand"),
        (F16c, "f16c"),
        (Fma, "fma"),
        (Pconfig,"pconfig"),
        (Wbnoinvd, "wbnoinvd"),
        (Fma4, "fma4"),
        (Prfchw,"prfchw"),
        (Rdpid, "rdpid"),
        (PrefetchWt11,"prefetchwt11"),
        (Rdseed, "rdseed"),
        (Sgx, "sgx"),
        (Xop, "xop"),
        (Lwp, "lwp"),
        (M3dNow, "3dnow"),
        (M3dNowA, "3dnowa"),
        (Popcnt, "popcnt"),
        (Abm, "abm"),
        (Adx, "adx"),
        (Bmi, "bmi"),
        (Bmi2, "bmi2"),
        (Lzcnt, "lzcnt"),
        (Fxsr, "fxsr"),
        (XSave, "xsave"),
        (XSaveOpt, "xsaveopt"),
        (XSaveC, "xsavec"),
        (XSaveS,"xsaves"),
        (Rtm, "rtm"),
        (Hle, "hle"),
        (Tbm, "tbm"),
        (MWaitX, "mwaitx"),
        (ClZero, "clzero"),
        (Pku, "pku"),
        (Avx512vbmi2, "avx512vbmi"),
        (Avx512bf16, "avx512bf16"),
        (Avx512fp16, "avx512fp16"),
        (Gfni, "gfni"),
        (Vaes, "vaes"),
        (WaitPkg, "waitpkg"),
        (VpclMulQdq, "vpclmulqdq"),
        (Avx512BitAlg, "avx512bitalg"),
        (MovDirI,"movdiri"),
        (MovDir64b, "movdir64b"),
        (Enqcmd, "enqcmd"),
        (Uintr, "uintr"),
        (Tsxldtrk, "tsxldtrk"),
        (Avx512VPopcntDq, "avx512vpopcntdq"),
        (Avx512Vp2Intersect, "avx512vp2intersect"),
        (Avx5124Fmaps, "avx5124fmaps"),
        (Avx512Vnni, "avx512vnni"),
        (AvxVnni, "avxvnni"),
        (Avx5124VnniW, "avx512fvnniw"),
        (ClDemote, "cldemote"),
        (Serialize, "serialize"),
        (AmxTile, "amx-tile"),
        (AmxInt8, "amx-int8"),
        (AmxBf16, "amx-bf16"),
        (HReset, "hreset"),
        (Kl, "kl"),
        (WideKl, "widekl"),
        (X87, "x87"),
        (Cx8, "cx8"),
        (Cx16, "cx16"),
        (Vme, "vme"),
        (Long, "long"),
        (X2Apic, "x2apic"),
        (Tsc, "tsc"),
        (Monitor, "monitor")
    }

    pub fn get_x86_features() -> HashSet<X86Feature> {
        let [hfc, ..] = super::cpuid(0, 0);

        let mut set = HashSet::<X86Feature>::new();

        if hfc >= 1 {
            let [_, _, features1, features2] = super::cpuid(1, 0);

            if (features1 & 1) != 0 {
                let _ = set.insert(X86Feature::X87);
            }

            if (features1 & 0x10) != 0 {
                let _ = set.insert(X86Feature::Tsc);
            }

            if (features1 & 0x100) != 0 {
                let _ = set.insert(X86Feature::Cx8);
            }
            if (features1 & 0x800000) != 0 {
                let _ = set.insert(X86Feature::Mmx);
            }
            if (features1 & 0x1000000) != 0 {
                let _ = set.insert(X86Feature::Fxsr);
            }
            if (features1 & 0x2000000) != 0 {
                let _ = set.insert(X86Feature::Sse);
            }
            if (features1 & 0x4000000) != 0 {
                let _ = set.insert(X86Feature::Sse2);
            }

            if (features2 & 0x80000000) == 0 {
                if (features2 & 0x04) != 0 {
                    let _ = set.insert(X86Feature::Monitor);
                }

                if (features2 & 0x01) != 0 {
                    let _ = set.insert(X86Feature::Sse3);
                }

                if (features2 & 0x200) != 0 {
                    let _ = set.insert(X86Feature::Ssse3);
                }

                if (features2 & 0x40000) != 0 {
                    let _ = set.insert(X86Feature::Sse4_1);
                }
                if (features2 & 0x80000) != 0 {
                    let _ = set.insert(X86Feature::Sse4_1);
                }

                if (features2 & 0x100000) != 0 {
                    let _ = set.insert(X86Feature::X2Apic);
                }

                if (features2 & 0x2000) != 0 {
                    let _ = set.insert(X86Feature::Cx16);
                }

                if (features2 & 0x10000000) != 0 {
                    let _ = set.insert(X86Feature::Avx);
                }

                if (features2 & 0x40000000) != 0 {
                    let _ = set.insert(X86Feature::Rdrand);
                }
            }

            if hfc >= 7 {
                let [_, features1, _, _] = super::cpuid(7, 0);
                if (features1 & 0x01) != 0 {
                    let _ = set.insert(X86Feature::FsGsBase);
                }

                if (features1 & 0x20) != 0 {
                    let _ = set.insert(X86Feature::Avx2);
                }

                if (features1 & 0x40000) != 0 {
                    let _ = set.insert(X86Feature::Rdseed);
                }
            }
        }

        set
    }
}

#[no_mangle]
pub unsafe extern "C" fn __has_x86_feature(f: StringView) -> bool {
    static FEATURES: OnceCell<HashSet<X86Feature>> = OnceCell::new();

    let feature = if let Ok(feature) = f.parse::<X86Feature>() {
        feature
    } else {
        return false;
    };

    FEATURES
        .get_or_init(features::get_x86_features)
        .contains(&feature)
}
