// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{CALLAPI_ELF, CALLAPI_ID};
use risc0_zkvm::{
    default_executor_from_elf,
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};
use reqwest;

fn main() {
    let binance_spot: Vec<BinanceSpot> = reqwest::get("https://api.binance.us/api/v3/ticker/price")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    // First, we construct an executor environment
    let env = ExecutorEnv::builder().add_input(&binance_spot).build().unwrap();
    let prover = default_prover();
    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();
    let receipt = prover.prove_elf(env, CALLAPI_ELF).unwrap();
    let outputs = from_slice(&receipt.journal).unwrap();
    // Next, we make an executor, loading the (renamed) ELF binary.
    // let mut exec = default_executor_from_elf(env, CALLAPI_ELF).unwrap();

    // Run the executor to produce a session.
    // let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    // let receipt = session.prove().unwrap();

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    // receipt.verify(CALLAPI_ID).unwrap();

    println!();
    println!("out  {:?}",outputs);

}
