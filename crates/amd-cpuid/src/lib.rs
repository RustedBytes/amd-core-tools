#![no_std]
//! CPUID-based AMD CPU and feature detection helpers.
//!
//! This crate only uses user-space CPUID leaves, so its public helpers are safe
//! to call from normal application code. On non-`x86_64` targets the helpers
//! return `false`.

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid;

/// Returns `true` when CPUID leaf 0 reports the `AuthenticAMD` vendor string.
pub fn is_amd_cpu() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // CPUID leaf 0 returns the vendor ID split across EBX, EDX, and ECX.
        let res = __cpuid(0);
        res.ebx == 0x6874_7541 && res.ecx == 0x444D_4163 && res.edx == 0x6974_6E65
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Returns `true` when SMT (Simultaneous Multithreading) is supported.
pub fn has_smt() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // Leaf 1 EDX[28] advertises hyper-threading / SMT capability.
        (__cpuid(0x0000_0001).edx & (1 << 28)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Returns `true` when AMD Secure Virtual Machine (SVM) support is present.
pub fn has_svm() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // Extended leaf 0x8000_0001 ECX[2] is AMD's SVM feature bit.
        (__cpuid(0x8000_0001).ecx & (1 << 2)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Returns `true` when Secure Memory Encryption (SME) is supported.
pub fn has_sme() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // Leaf 0x8000_001F EAX[0] reports SME capability.
        (__cpuid(0x8000_001F).eax & (1 << 0)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Returns `true` when Secure Encrypted Virtualization (SEV) is supported.
pub fn has_sev() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // Leaf 0x8000_001F EAX[1] reports SEV capability.
        (__cpuid(0x8000_001F).eax & (1 << 1)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Returns `true` when SEV Encrypted State (SEV-ES) is supported.
pub fn has_sev_es() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // Leaf 0x8000_001F EAX[3] reports SEV-ES capability.
        (__cpuid(0x8000_001F).eax & (1 << 3)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Returns `true` when SEV Secure Nested Paging (SEV-SNP) is supported.
pub fn has_sev_snp() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // Leaf 0x8000_001F EAX[4] reports SEV-SNP capability.
        (__cpuid(0x8000_001F).eax & (1 << 4)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Returns `true` when Enhanced Predictive Store Forwarding (EPSF) is supported.
pub fn has_epsf() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // Leaf 0x8000_0021 EAX[18] reports EPSF capability.
        (__cpuid(0x8000_0021).eax & (1 << 18)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Returns `true` when Supervisor Mode Execution Prevention (SMEP) is supported.
pub fn has_smep() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // Structured extended leaf 7 EBX[7] reports SMEP capability.
        (__cpuid(0x0000_0007).ebx & (1 << 7)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Returns `true` when Supervisor Mode Access Prevention (SMAP) is supported.
pub fn has_smap() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        // Structured extended leaf 7 EBX[20] reports SMAP capability.
        (__cpuid(0x0000_0007).ebx & (1 << 20)) != 0
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpuid_safe_execution() {
        let _ = is_amd_cpu();
        let _ = has_smt();
        let _ = has_svm();
        let _ = has_epsf();
        assert!(true);
    }
}
