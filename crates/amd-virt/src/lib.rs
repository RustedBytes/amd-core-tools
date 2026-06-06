#![no_std]
//! AMD SVM virtualization helpers.
//!
//! This crate exposes low-level register and instruction helpers for privileged
//! code. Calling these functions from user space will fault on normal systems.

use core::arch::asm;

/// Extended Feature Enable Register.
pub const MSR_EFER: u32 = 0xC000_0080;
/// Virtual Machine Control Register.
pub const MSR_VM_CR: u32 = 0xC001_0114;

/// EFER bit that enables AMD Secure Virtual Machine instructions.
pub const EFER_SVME_BIT: u64 = 1 << 12;
/// VM_CR bit set by firmware when SVM is disabled.
pub const VM_CR_SVMDIS_BIT: u64 = 1 << 4;

/// AMD Secure Processor Mailbox structure used for Tiered Memory Page Migration (TMPM).
#[repr(C)]
#[derive(Debug)]
pub struct AspMailboxBuffer {
    /// Total mailbox buffer size in bytes.
    pub total_size: u32,
    /// Firmware-filled command status.
    pub status: u32,
    /// Low 32 bits of the image address.
    pub image_addr_lo: u32,
    /// High 32 bits of the image address.
    pub image_addr_hi: u32,
    /// Image size in bytes.
    pub image_size: u32,
    /// Reserved field kept for ABI layout compatibility.
    pub reserved: u32,
}

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

/// Checks if BIOS has locked SVM. REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 on hardware where `MSR_VM_CR` is available.
/// Executing `rdmsr` without sufficient privilege or on an unsupported MSR can
/// fault.
pub unsafe fn is_svm_disabled_by_bios() -> bool {
    let vm_cr = read_msr(MSR_VM_CR);
    // Firmware sets SVMDIS when SVM cannot be enabled by software.
    (vm_cr & VM_CR_SVMDIS_BIT) != 0
}

/// Enables SVM by setting SVME in the EFER register. REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 on an AMD CPU that supports SVM, must ensure SVM
/// is not firmware-disabled, and must ensure modifying EFER is valid for the
/// current kernel or hypervisor state.
pub unsafe fn enable_svm() {
    let efer = read_msr(MSR_EFER);
    // Preserve all existing EFER bits and set only SVME.
    write_msr(MSR_EFER, efer | EFER_SVME_BIT);
}

/// Triggers VMRUN. REQUIRES RING 0.
///
/// # Safety
///
/// The caller must run at CPL 0 with SVM enabled and must pass the physical
/// address of a valid, correctly initialized VMCB. An invalid VMCB or address
/// can corrupt guest or host state and fault the processor.
pub unsafe fn vmrun(vmcb_physical_address: u64) {
    // VMRUN expects RAX to contain the physical address of the VMCB.
    asm!("vmrun", in("rax") vmcb_physical_address, options(nostack));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_virt_constants() {
        assert_eq!(MSR_EFER, 0xC000_0080);
        assert_eq!(core::mem::size_of::<AspMailboxBuffer>(), 24);
    }
}
