#![no_std]
//! AMD performance and speculation-control helpers.
//!
//! The functions in this crate modify machine-specific registers and are meant
//! for privileged system software, not normal user-space programs.

use core::arch::asm;

/// Hardware Configuration MSR containing the Core Performance Boost disable bit.
pub const MSR_HWCR: u32 = 0xC001_0015;
/// Speculation Control MSR containing PSF and EPSF control bits.
pub const MSR_SPEC_CTRL: u32 = 0x0000_0048;

/// HWCR bit that disables Core Performance Boost when set.
pub const HWCR_CPB_DIS_BIT: u64 = 1 << 25;
/// SPEC_CTRL bit that disables Predictive Store Forwarding when set.
pub const SPEC_CTRL_PSFD_BIT: u64 = 1 << 7;

#[inline(always)]
unsafe fn read_msr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    // RDMSR reads EDX:EAX from the MSR selected in ECX.
    asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high, options(nomem, nostack, preserves_flags));
    ((high as u64) << 32) | (low as u64)
}

#[inline(always)]
unsafe fn write_msr(msr: u32, value: u64) {
    let low = (value & 0xFFFF_FFFF) as u32;
    let high = (value >> 32) as u32;
    // WRMSR writes EDX:EAX to the MSR selected in ECX.
    asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high, options(nomem, nostack, preserves_flags));
}

/// Enables Core Performance Boost (CPB). REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 on hardware where `MSR_HWCR` is available and
/// must ensure changing CPB policy is valid for the platform power, thermal,
/// and firmware configuration.
pub unsafe fn enable_cpb() {
    let hwcr = read_msr(MSR_HWCR);
    // CPB is enabled by clearing the CPB disable bit.
    write_msr(MSR_HWCR, hwcr & !HWCR_CPB_DIS_BIT);
}

/// Disables Predictive Store Forwarding (PSF & EPSF) via SPEC_CTRL. REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 on hardware where `MSR_SPEC_CTRL` is available
/// and must ensure changing PSF behavior is appropriate for all running code.
pub unsafe fn disable_psf() {
    let spec = read_msr(MSR_SPEC_CTRL);
    // Setting PSFD disables Predictive Store Forwarding.
    write_msr(MSR_SPEC_CTRL, spec | SPEC_CTRL_PSFD_BIT);
}

/// Enables Predictive Store Forwarding. REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 on hardware where `MSR_SPEC_CTRL` is available
/// and must ensure re-enabling PSF does not violate the system's security
/// policy or mitigation requirements.
pub unsafe fn enable_psf() {
    let spec = read_msr(MSR_SPEC_CTRL);
    // Clearing PSFD allows Predictive Store Forwarding again.
    write_msr(MSR_SPEC_CTRL, spec & !SPEC_CTRL_PSFD_BIT);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_perf_constants() {
        assert_eq!(MSR_SPEC_CTRL, 0x0000_0048);
        assert_eq!(SPEC_CTRL_PSFD_BIT, 128);
    }
}
