### 1) Consensus RPC Setup

To run Risc0 Helios, you need a Beacon Chain node for your source chain. For example, to run an Ethereum mainnet light client, you need an Ethereum mainnet beacon node.

The beacon chain node must support the RPC methods for the [Altair light client protocol](https://github.com/ethereum/consensus-specs/blob/dev/specs/altair/light-client/sync-protocol.md). As of 10/15/24, Nimbus is the only consensus client that supports these "light sync" endpoints by default.

There are a few options for setting up a consensus RPC with "light sync" endpoints:

1. Get an RPC from a provider running Nimbus nodes. [Chainstack](https://chainstack.com/) is currently the only provider we're aware of that supports this. Set up a node on Chainstack and use the consensus client endpoint for an Ethereum mainnet node.
2. Run a Nimbus eth2 beacon node. Instructions [here](https://nimbus.guide/el-light-client.html).
3. There is a community-maintained list of Ethereum Beacon Chain light sync endpoints [here](https://s1na.github.io/light-sync-endpoints). These endpoints are not guaranteed to work, and are often unreliable.

The RPC you just set up will be used as the `SOURCE_CONSENSUS_RPC_URL` in the next step.

### 2) Environment Setup

In the root directory, create a file called `.env` (mirroring `.env.example`) and set the following environment variables:

| Parameter | Description |
|-----------|-------------|
| `SOURCE_CHAIN_ID` | Chain ID for the source chain. |
| `SOURCE_CONSENSUS_RPC_URL` | RPC URL for the source chain. See how to get this in the [Consensus RPC Setup](#1-consensus-rpc-setup) section |