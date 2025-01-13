use alloy_primitives::FixedBytes;
use anyhow::Result;
use clap::{command, Parser};
use helios_consensus_core::consensus_spec::MainnetConsensusSpec;
use helios_ethereum::{
    consensus::Inner,
    rpc::{http_rpc::HttpRpc, ConsensusRpc},
};
use risc0_helios_guest_wrapper::GUEST_ELF;
use risc0_helios_primitives::{
    ProofInputs, {FinalityUpdate, Update},
};
use risc0_helios_script::{get_client, get_latest_checkpoint, get_updates};
use risc0_zkvm::{default_prover, ExecutorEnv};

#[derive(Parser, Debug, Clone)]
#[command(about = "Get the genesis parameters from a block.")]
pub struct GenesisArgs {
    #[arg(long)]
    pub slot: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = GenesisArgs::parse();

    let checkpoint = get_checkpoint(args).await;
    let helios_client = get_client(checkpoint).await;
    let sync_committee_updates = get_sync_committee_updates(&helios_client).await;
    let finality_update = get_finality_update(&helios_client).await;
    let expected_current_slot = helios_client.expected_current_slot();

    let inputs = ProofInputs {
        sync_committee_updates,
        finality_update,
        expected_current_slot,
        state: helios_client.store.into(),
        genesis_root: helios_client.config.chain.genesis_root,
        forks: helios_client.config.forks.clone(),
    };

    let env = ExecutorEnv::builder()
        .write(&inputs)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    let proof = prover.prove(env, &GUEST_ELF.elf).unwrap();
    dbg!(&proof.stats);

    Ok(())
}

async fn get_checkpoint(args: GenesisArgs) -> FixedBytes<32> {
    if let Some(slot) = args.slot {
        risc0_helios_script::get_checkpoint(slot).await
    } else {
        get_latest_checkpoint().await
    }
}

async fn get_sync_committee_updates(
    client: &Inner<MainnetConsensusSpec, HttpRpc>,
) -> Vec<Update<MainnetConsensusSpec>> {
    get_updates(client)
        .await
        .into_iter()
        .map(Into::into)
        .collect()
}

async fn get_finality_update(
    client: &Inner<MainnetConsensusSpec, HttpRpc>,
) -> FinalityUpdate<MainnetConsensusSpec> {
    client.rpc.get_finality_update().await.unwrap().into()
}
