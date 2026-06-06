use amd_cpuid::*;
use amd_legacy::*;
use amd_rng::*;

fn main() {
    println!("=== AMD Core Features Workspace ===");

    if !is_amd_cpu() {
        println!("Not an AuthenticAMD CPU. Exiting...");
        return;
    }

    // 1. CPUID & Features
    println!("SMT Supported: {}", has_smt());
    println!("SME Supported: {}", has_sme());
    println!("SEV Supported: {}", has_sev());
    println!("SEV-SNP Supported: {}", has_sev_snp());
    println!("EPSF Supported: {}", has_epsf());

    // 2. Hardware RNG
    match get_rdrand() {
        Ok(val) => println!("Hardware RDRAND: 0x{:016X}", val),
        Err(e) => println!("RDRAND failed: {:?}", e),
    }

    // 3. Legacy x87 (Safe in Ring 3)
    unsafe {
        init_x87();
        println!("x87 FPU Initialized.");
    }

    // WARNING: The following operations require RING 0 (Kernel) privileges.
    // Uncommenting them in user-space will crash the program.

    /*
    unsafe {
        // 4. Security & Encrypted Virtualization
        if has_sme() || has_sev() {
            let sev_status = get_sev_status();
            println!("SEV Status: {:?}", sev_status);
        }

        // 5. Execution Protections
        if has_smep() && has_smap() {
            enable_smep_smap();
        }

        // 6. Performance (CPB & PSF)
        enable_cpb();
        if has_epsf() {
            disable_psf(); // Disables Predictive Store Forwarding for security
        }

        // 7. Virtualization & TMPM
        if has_svm() && !is_svm_disabled_by_bios() {
            enable_svm();
        }

        // 8. Trigger SMM (System Management Mode) via Software SMI
        // trigger_sw_smi(0x01);
    }
    */
}
