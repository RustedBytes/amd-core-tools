#![no_std]
//! Legacy x86 processor helpers used by the AMD core tools workspace.
//!
//! Some helpers, such as x87 state access, are usable from normal user space.
//! Platform-specific port I/O helpers require elevated privileges.

use core::arch::asm;

/// Reads the value of a General-Purpose Register (RBP as an example).
pub fn read_general_purpose_register() -> u64 {
    let val: u64;
    unsafe {
        // RBP is used here as a simple example of moving a GPR into a Rust value.
        asm!("mov {}, rbp", out(reg) val, options(nomem, nostack, preserves_flags));
    }
    val
}

/// Initializes the x87 Floating-Point Unit (FPU).
/// Reverts the FPU back to its default state. Safe in User Space.
///
/// # Safety
///
/// The caller must ensure resetting the current thread's x87 state will not
/// violate any active floating-point context assumptions.
pub unsafe fn init_x87() {
    // FINIT resets the x87 control, status, tag, instruction, and data pointers.
    asm!("finit", options(nomem, nostack));
}

/// Reads the x87 control word. Safe in User Space.
///
/// # Safety
///
/// The caller must ensure the current execution environment permits x87
/// instructions and that reading the thread-local x87 state is appropriate.
pub unsafe fn read_x87_control_word() -> u16 {
    let mut cw: u16 = 0;
    // FNSTCW stores the thread's current x87 control word to memory.
    asm!("fnstcw word ptr [{}]", in(reg) &mut cw, options(nostack));
    cw
}

/// Triggers a Software System Management Interrupt (SMI) to enter SMM.
/// Most AMD APM platforms map the Software SMI command port to 0xB2.
/// REQUIRES I/O PRIVILEGE LEVEL (Ring 0).
///
/// # Safety
///
/// The caller must have permission to perform port I/O and must know that
/// issuing this platform-specific SMI command is supported and safe for the
/// running firmware and hardware state.
pub unsafe fn trigger_sw_smi(command: u8) {
    // Many PC-compatible platforms use I/O port 0xB2 as the software SMI command port.
    asm!(
        "out dx, al",
        in("dx") 0xB2u16,
        in("al") command,
        options(nomem, nostack, preserves_flags)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_legacy_x87() {
        // x87 is ring 3 safe
        unsafe {
            init_x87();
            let cw = read_x87_control_word();
            assert_ne!(cw, 0); // CW is typically 0x037F after init
        }
    }
}
