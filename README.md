This repository is not a minimal setup for running Helios in Risc0. Instead, it is a modified sp1-helios repository that has been adapted to run Helios in Risc0.

### 1) Running the Project
To run the program, simply execute:

```bash
Copy code
cargo run --bin main
```


### 2) Consensus RPC Setup

To run Risc0 Helios, you need a Beacon Chain node for your source chain. For example, to run an Ethereum mainnet light client, you need an Ethereum mainnet beacon node.

The beacon chain node must support the RPC methods for the [Altair light client protocol](https://github.com/ethereum/consensus-specs/blob/dev/specs/altair/light-client/sync-protocol.md). As of 10/15/24, Nimbus is the only consensus client that supports these "light sync" endpoints by default.

There are a few options for setting up a consensus RPC with "light sync" endpoints:

1. Get an RPC from a provider running Nimbus nodes. [Chainstack](https://chainstack.com/) is currently the only provider we're aware of that supports this. Set up a node on Chainstack and use the consensus client endpoint for an Ethereum mainnet node.
2. Run a Nimbus eth2 beacon node. Instructions [here](https://nimbus.guide/el-light-client.html).
3. There is a community-maintained list of Ethereum Beacon Chain light sync endpoints [here](https://s1na.github.io/light-sync-endpoints). These endpoints are not guaranteed to work, and are often unreliable.

The RPC you just set up will be used as the `SOURCE_CONSENSUS_RPC_URL` in the next step.

### 3) Environment Setup

In the root directory, there is a file called `.env` with the following environment variables (already filled):

| Parameter | Description |
|-----------|-------------|
| `SOURCE_CHAIN_ID` | Chain ID for the source chain. |
| `SOURCE_CONSENSUS_RPC_URL` | RPC URL for the source chain. See how to get this in the [Consensus RPC Setup](#1-consensus-rpc-setup) section |

### 4) Output

When you run the process, you can expect output similar to the following:

```
WARNING: proving in dev mode. This will not generate valid, secure proofs.
Processing update 1 of 1.
Update 1 is valid.
Finality update is valid.
Finality update applied.
Head: 10829504
Exec state root: 0x855e15e26223c6f3a8f8f72a37ebb0681192800ab7a7dc61277e2035df01297f
WARNING: Proving in dev mode does not generate a valid receipt. Receipts generated from this process are invalid and should never be used in production.
[script/./bin/test.rs:51:5] &proof.stats = SessionStats {
    segments: 7670,
    total_cycles: 8042577920,
    user_cycles: 7093294775,
    paging_cycles: 921248146,
    reserved_cycles: 28034999,
}
```