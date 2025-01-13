This repository is not a minimal setup for running Helios in Risc0. Instead, it is a modified [sp1-helios](https://github.com/succinctlabs/sp1-helios) repository that has been adapted to run Helios in Risc0.

### 1) Running the Project
To run the program, execute:

```bash
RISC0_DEV_MODE=1 cargo run --bin main
```

The execution may take about 3 minutes.

### 2) Output

When you run the process, you can expect output similar to the following:

```
WARNING: proving in dev mode. This will not generate valid, secure proofs.
Processing update 1 of 1.
Update 1 is valid.
Finality update is valid.
Finality update applied.
WARNING: Proving in dev mode does not generate a valid receipt. Receipts generated from this process are invalid and should never be used in production.
[script/./bin/test.rs:51:5] &proof.stats = SessionStats {
    segments: 7670,
    total_cycles: 8042577920,
    user_cycles: 7093294775,
    paging_cycles: 921248146,
    reserved_cycles: 28034999,
}
```
