#![no_std]
//! Hardware random number helpers backed by x86_64 `RDRAND` and `RDSEED`.
//!
//! The functions retry a small number of times because both instructions can
//! legally report that no value is currently available.

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{_rdrand64_step, _rdseed64_step};

/// Errors returned when a hardware random instruction cannot produce a value.
#[derive(Debug, PartialEq)]
pub enum RngError {
    /// The CPU instruction was available but did not return a value after retries.
    HardwareNotReady,
    /// The current target architecture does not expose the required intrinsic.
    UnsupportedArchitecture,
}

/// Reads a 64-bit value from `RDRAND`.
///
/// The instruction returns values from the CPU's hardware random number
/// generator. This function tries up to ten times before returning
/// [`RngError::HardwareNotReady`].
pub fn get_rdrand() -> Result<u64, RngError> {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let mut value: u64 = 0;
        for _ in 0..10 {
            // A return value of 1 means the carry flag was set and `value` is valid.
            if _rdrand64_step(&mut value) == 1 {
                return Ok(value);
            }
        }
        Err(RngError::HardwareNotReady)
    }
    #[cfg(not(target_arch = "x86_64"))]
    Err(RngError::UnsupportedArchitecture)
}

/// Reads a 64-bit seed value from `RDSEED`.
///
/// `RDSEED` is intended for seeding deterministic random bit generators. Like
/// `RDRAND`, it may need retries when the hardware seed source is temporarily
/// empty.
pub fn get_rdseed() -> Result<u64, RngError> {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let mut value: u64 = 0;
        for _ in 0..10 {
            // A return value of 1 means the carry flag was set and `value` is valid.
            if _rdseed64_step(&mut value) == 1 {
                return Ok(value);
            }
        }
        Err(RngError::HardwareNotReady)
    }
    #[cfg(not(target_arch = "x86_64"))]
    Err(RngError::UnsupportedArchitecture)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rng_execution() {
        let _ = get_rdrand();
        let _ = get_rdseed();
    }
}
