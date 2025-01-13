#![no_main]

use alloy_primitives::{B256, U256};
use helios_consensus_core::{
    apply_finality_update, apply_update, consensus_spec::MainnetConsensusSpec,
    verify_finality_update, verify_update,
};
use risc0_helios_primitives::{init_store, ProofInputs, ProofOutputs};
use tree_hash::TreeHash;

/// Program flow:
/// 1. Apply sync committee updates, if any
/// 2. Apply finality update
/// 3. Verify execution state root proof
/// 4. Asset all updates are valid
/// 5. Commit new state root, header, and sync committee for usage in the on-chain contract
pub fn main(input: ProofInputs) -> ProofOutputs {
    let ProofInputs {
        sync_committee_updates,
        finality_update,
        expected_current_slot,
        state,
        genesis_root,
        forks,
    } = input;
    let finality_update = finality_update.into();
    let sync_committee_updates: Vec<helios_consensus_core::types::Update<MainnetConsensusSpec>> =
        sync_committee_updates.into_iter().map(Into::into).collect();

    let mut store = init_store(state);

    let prev_header: B256 = store.finalized_header.beacon().tree_hash_root();
    let prev_head = store.finalized_header.beacon().slot;

    // 1. Apply sync committee updates, if any
    for (index, update) in sync_committee_updates.iter().enumerate() {
        println!(
            "Processing update {} of {}.",
            index + 1,
            sync_committee_updates.len()
        );
        let update_is_valid =
            verify_update(update, expected_current_slot, &store, genesis_root, &forks).is_ok();

        if !update_is_valid {
            panic!("Update {} is invalid!", index + 1);
        }
        println!("Update {} is valid.", index + 1);
        apply_update(&mut store, update);
    }

    // 2. Apply finality update
    let finality_update_is_valid = verify_finality_update(
        &finality_update,
        expected_current_slot,
        &store,
        genesis_root,
        &forks,
    )
    .is_ok();
    if !finality_update_is_valid {
        panic!("Finality update is invalid!");
    }
    println!("Finality update is valid.");

    apply_finality_update(&mut store, &finality_update);
    println!("Finality update applied.");

    // 3. Commit new state root, header, and sync committee for usage in the on-chain contract
    let header: B256 = store.finalized_header.beacon().tree_hash_root();
    let sync_committee_hash: B256 = store.current_sync_committee.tree_hash_root();
    let next_sync_committee_hash: B256 = match &mut store.next_sync_committee {
        Some(next_sync_committee) => next_sync_committee.tree_hash_root(),
        None => B256::ZERO,
    };
    let head = store.finalized_header.beacon().slot;

    let proof_outputs = ProofOutputs {
        execution_state_root: *store
            .finalized_header
            .execution()
            .expect("Execution payload doesn't exist.")
            .state_root(),
        new_header: header,
        next_sync_committee_hash,
        new_head: U256::from(head),
        prev_header,
        prev_head: U256::from(prev_head),
        sync_committee_hash,
    };
    proof_outputs
}
