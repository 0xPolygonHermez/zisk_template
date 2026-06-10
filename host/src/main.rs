//! Host program (untrusted): Orchestrates the entire proving pipeline (build, setup, prove)
//! and supplies the guest program inputs.

use zisk_sdk::{GuestProgram, load_program};

/// Guest ELF binary, embedded into the host at build time.
static PROGRAM: GuestProgram = load_program!("template-guest");

fn main() -> anyhow::Result<()> {
    
    Ok(())
    
}
