use anyhow::Result;
use zisk_sdk::{GuestProgram, ProverClient, ZiskStdin, load_program};

static PROGRAM: GuestProgram = load_program!("guest");

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting ZisK Prover Client...");

    let client = ProverClient::embedded()
        .build()?;

    client.upload(&PROGRAM).run()?;
    client.setup(&PROGRAM).run()?.await?;

    let n = 1000u32;
    let stdin = ZiskStdin::new();
    stdin.write(&n);

    let handle = client
        .execute(&PROGRAM, stdin.clone())
        .run()?;
    let result = handle.await?; // automatically calls finish() on the stream

    println!(
        "ZisK has executed program with {} cycles in {:?} ms",
        result.get_execution_steps(),
        result.get_execution_time()
    );

    let prove_handle = client
        .prove(&PROGRAM, stdin.clone())
        .run()?;
    let vadcop_result = prove_handle.await?;

    let vkey = PROGRAM.vk()?;
    vadcop_result.with_program_vk(&vkey).verify()?;

    println!("successfully generated and verified proof for the program!");
    println!("Running second proof generation with new input...");

    let prove_handle2 = client
        .prove(&PROGRAM, stdin.clone())
        .run()?;
    let vadcop_result2 = prove_handle2.await?;

    let vkey = PROGRAM.vk()?;
    vadcop_result2.with_program_vk(&vkey).verify()?;

    println!("successfully generated and verified proof for the program!");

    Ok(())
}
