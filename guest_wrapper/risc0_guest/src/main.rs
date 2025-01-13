#![no_main]

risc0_zkvm::guest::entry!(main);

use guest::main as guest_main;
use risc0_helios_primitives::ProofInputs;
use risc0_zkvm::guest::env;

fn main() {
    let input: ProofInputs = env::read();
    let guest_output = guest_main(input);
    env::commit(&guest_output);
}
