#![no_std]
//! AMD Secure Encrypted Virtualization (SEV) and SME status helpers.
//!
//! The public readers in this crate execute `rdmsr`, which is privileged on
//! x86_64. They are intended for kernels, hypervisors, or similarly privileged
//! environments.

use bitflags::bitflags;
use core::arch::asm;

/// Machine-specific register containing SEV enablement status bits.
pub const MSR_SEV_STATUS: u32 = 0xC001_0131;
/// System configuration MSR containing the SME enable bit.
pub const MSR_SYSCFG: u32 = 0xC001_0010;

bitflags! {
    /// Status bits read from [`MSR_SEV_STATUS`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SevStatusFlags: u64 {
        /// SEV is enabled for the current context.
        const SEV_ENABLED          = 1 << 0;
        /// SEV-ES is enabled for the current context.
        const SEV_ES_ENABLED       = 1 << 1;
        /// SEV-SNP is enabled for the current context.
        const SEV_SNP_ENABLED      = 1 << 2;
    }
}

bitflags! {
    /// System configuration bits read from [`MSR_SYSCFG`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SysCfgFlags: u64 {
        /// Secure Memory Encryption Enable, used by SME and TSME.
        const SMEE = 1 << 23;
    }
}

#[inline(always)]
unsafe fn read_msr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    // RDMSR reads EDX:EAX from the MSR selected in ECX.
    asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags));
    ((high as u64) << 32) | (low as u64)
}

/// Retrieves the current SEV status of the processor. REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 on hardware where `MSR_SEV_STATUS` is available.
/// Executing `rdmsr` without sufficient privilege or on an unsupported MSR can
/// fault.
pub unsafe fn get_sev_status() -> SevStatusFlags {
    let val = read_msr(MSR_SEV_STATUS);
    // Ignore reserved or unknown bits so newer hardware remains readable.
    SevStatusFlags::from_bits_truncate(val)
}

/// Checks if TSME (Transparent Secure Memory Encryption) is enabled. REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 on hardware where `MSR_SYSCFG` is available.
/// Executing `rdmsr` without sufficient privilege or on an unsupported MSR can
/// fault.
pub unsafe fn is_tsme_enabled() -> bool {
    let syscfg = read_msr(MSR_SYSCFG);
    // SMEE gates Secure Memory Encryption, including transparent SME modes.
    (syscfg & SysCfgFlags::SMEE.bits()) != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sev_flags() {
        assert_eq!(SysCfgFlags::SMEE.bits(), 1 << 23);
        assert_eq!(SevStatusFlags::SEV_SNP_ENABLED.bits(), 1 << 2);
    }
}
