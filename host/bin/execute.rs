use anyhow::Result;
use common::Output;
use zisk_sdk::{GuestProgram, ProverClient, ZiskStdin, load_program};

static PROGRAM: GuestProgram = load_program!("guest");

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting ZisK Prover Client...");

    // Create an input stream and write '1000' to it.
    let n = 1000u32;
    let stdin = ZiskStdin::new();
    stdin.write(&n);
    println!("Input prepared: {} iterations", n);

    // Create a `ProverClient` method.
    println!("Building prover client...");
    let client = ProverClient::embedded()
        .build()?;

    println!("Setting up program...");
    client.setup(&PROGRAM).run()?.await?; // S'ha de fer un must use
    println!("Setup completed successfully");

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    println!("Executing program (no proof generation)...");
    let result = client.execute(&PROGRAM, stdin.clone()).run()?.await?;

    println!("\u{2713} Execution completed successfully!");
    println!("Cycles: {}", result.get_execution_steps());
    println!("Duration: {:?}", result.get_execution_time());

    println!("Reading public outputs...");
    let output: Output = result.get_public_values_abi()?;
    println!("Public outputs:");
    println!("  Hash: {:02x?}", output.hash);
    println!("  Iterations: {}", output.iterations);
    println!("  Magic number: 0x{:08x}", output.magic_number);

    Ok(())
}
