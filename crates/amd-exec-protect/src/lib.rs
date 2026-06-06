#![no_std]
//! Execution and memory protection helpers for x86_64 control registers.
//!
//! These helpers write CR4 and therefore require privileged execution. They are
//! intended for kernels, hypervisors, or controlled low-level experiments.

use core::arch::asm;

/// CR4 bit enabling Supervisor Mode Execution Prevention.
pub const CR4_SMEP_BIT: u64 = 1 << 20;
/// CR4 bit enabling Supervisor Mode Access Prevention.
pub const CR4_SMAP_BIT: u64 = 1 << 21;
/// CR4 bit enabling Memory Protection Keys.
pub const CR4_PKE_BIT: u64 = 1 << 22;

#[inline(always)]
unsafe fn read_cr4() -> u64 {
    let val: u64;
    // CR4 contains processor control bits for supervisor memory behavior.
    asm!("mov {}, cr4", out(reg) val, options(nomem, nostack, preserves_flags));
    val
}

#[inline(always)]
unsafe fn write_cr4(val: u64) {
    // CR4 writes take effect immediately, so callers must prepare mappings first.
    asm!("mov cr4, {}", in(reg) val, options(nomem, nostack, preserves_flags));
}

/// Enables SMEP and SMAP. REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 on an x86_64 CPU where CR4 writes are valid.
/// Enabling these bits changes supervisor memory access semantics and can fault
/// immediately if the current kernel mappings or handlers are not prepared.
pub unsafe fn enable_smep_smap() {
    let cr4 = read_cr4();
    // Preserve existing control bits and set only SMEP and SMAP.
    write_cr4(cr4 | CR4_SMEP_BIT | CR4_SMAP_BIT);
}

/// Enables Memory Protection Keys. REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 on an x86_64 CPU that supports CR4.PKE and has
/// configured page tables and PKRU handling consistently with MPK semantics.
pub unsafe fn enable_mpk() {
    let cr4 = read_cr4();
    // Preserve existing control bits and set PKE.
    write_cr4(cr4 | CR4_PKE_BIT);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cr4_bitmasks() {
        assert_eq!(CR4_SMEP_BIT, 1 << 20);
        assert_eq!(CR4_PKE_BIT, 1 << 22);
    }
}
