use alloy_primitives::{Address, B256, U256};
use helios_consensus_core::consensus_spec::{ConsensusSpec, MainnetConsensusSpec};
use helios_consensus_core::types as helios_types;
use helios_consensus_core::types::bytes::ByteList;
use helios_consensus_core::types::Forks;
use helios_consensus_core::types::SyncCommittee;
use helios_consensus_core::types::{BeaconBlockHeader, LogsBloom, SyncAggregate};
use serde::{Deserialize, Serialize};
use ssz_rs::prelude::*;
pub use ssz_rs::prelude::{Bitvector, Vector};

use ssz_types::FixedVector;
use superstruct::superstruct;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProofInputs {
    pub sync_committee_updates: Vec<Update<MainnetConsensusSpec>>,
    pub finality_update: FinalityUpdate<MainnetConsensusSpec>,
    pub expected_current_slot: u64,
    pub state: State<MainnetConsensusSpec>,
    pub genesis_root: B256,
    pub forks: Forks,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ExecutionStateProof {
    #[serde(rename = "executionStateRoot")]
    pub execution_state_root: B256,
    #[serde(rename = "executionStateBranch")]
    pub execution_state_branch: Vec<B256>,
    pub gindex: String,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ProofOutputs {
    pub execution_state_root: B256,
    pub new_header: B256,
    pub next_sync_committee_hash: B256,
    pub new_head: U256,
    pub prev_header: B256,
    pub prev_head: U256,
    pub sync_committee_hash: B256,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct State<S: ConsensusSpec> {
    pub finalized_header: LightClientHeader,
    pub current_sync_committee: SyncCommittee<S>,
    pub next_sync_committee: Option<SyncCommittee<S>>,
}

impl<S: ConsensusSpec> From<helios_consensus_core::types::LightClientStore<S>> for State<S> {
    fn from(store: helios_consensus_core::types::LightClientStore<S>) -> Self {
        Self {
            finalized_header: store.finalized_header.into(),
            current_sync_committee: store.current_sync_committee,
            next_sync_committee: store.next_sync_committee,
        }
    }
}

/// When initializing `LightClientStore` from `State`, certain fields are set to default values
/// for the following reasons:
///
/// `optimistic_header`: Used in the `Inner.send_blocks()` function. Since we do not use this function,
/// the value of `optimistic_header` is irrelevant to us.
///
/// `previous_max_active_participants` and `current_max_active_participants`: Both fields are used in
/// the `safety_threshold(store)` function, which determines whether `optimistic_header` needs to be
/// updated. Because the actual value of `optimistic_header` is irrelevant to us, we do not
/// concern ourselves with the above-mentioned values.
///
/// `best_valid_update`: Used in the `force_update(store, current_slot)` function. Since we do not
/// call this function, the actual value of `best_valid_update` does not matter to us.
///
pub fn init_store<S: ConsensusSpec>(
    state: State<S>,
) -> helios_consensus_core::types::LightClientStore<S> {
    helios_consensus_core::types::LightClientStore {
        finalized_header: state.finalized_header.into(),
        current_sync_committee: state.current_sync_committee,
        next_sync_committee: state.next_sync_committee,
        optimistic_header: helios_consensus_core::types::LightClientHeader::default(),
        previous_max_active_participants: 0,
        current_max_active_participants: 0,
        best_valid_update: None,
    }
}

#[superstruct(
    variants(Bellatrix, Capella, Deneb),
    variant_attributes(
        derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq),
        serde(deny_unknown_fields),
    )
)]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ExecutionPayloadHeader {
    pub parent_hash: B256,
    pub fee_recipient: Address,
    pub state_root: B256,
    pub receipts_root: B256,
    pub logs_bloom: LogsBloom,
    pub prev_randao: B256,
    // #[serde(with = "serde_utils::u64")]
    pub block_number: u64,
    // #[serde(with = "serde_utils::u64")]
    pub gas_limit: u64,
    // #[serde(with = "serde_utils::u64")]
    pub gas_used: u64,
    // #[serde(with = "serde_utils::u64")]
    pub timestamp: u64,
    pub extra_data: ByteList<typenum::U32>,
    // #[serde(with = "serde_utils::u256")]
    pub base_fee_per_gas: U256,
    pub block_hash: B256,
    pub transactions_root: B256,
    #[superstruct(only(Capella, Deneb))]
    pub withdrawals_root: B256,
    #[superstruct(only(Deneb))]
    pub blob_gas_used: u64,
    #[superstruct(only(Deneb))]
    pub excess_blob_gas: u64,
}

impl Default for ExecutionPayloadHeader {
    fn default() -> Self {
        ExecutionPayloadHeader::Bellatrix(ExecutionPayloadHeaderBellatrix::default())
    }
}

impl From<ExecutionPayloadHeaderBellatrix> for helios_types::ExecutionPayloadHeaderBellatrix {
    fn from(header: ExecutionPayloadHeaderBellatrix) -> Self {
        helios_types::ExecutionPayloadHeaderBellatrix {
            parent_hash: header.parent_hash,
            fee_recipient: header.fee_recipient,
            state_root: header.state_root,
            receipts_root: header.receipts_root,
            logs_bloom: header.logs_bloom,
            prev_randao: header.prev_randao,
            block_number: header.block_number,
            gas_limit: header.gas_limit,
            gas_used: header.gas_used,
            timestamp: header.timestamp,
            extra_data: header.extra_data,
            base_fee_per_gas: header.base_fee_per_gas,
            block_hash: header.block_hash,
            transactions_root: header.transactions_root,
        }
    }
}

impl From<ExecutionPayloadHeaderCapella> for helios_types::ExecutionPayloadHeaderCapella {
    fn from(header: ExecutionPayloadHeaderCapella) -> Self {
        helios_types::ExecutionPayloadHeaderCapella {
            parent_hash: header.parent_hash,
            fee_recipient: header.fee_recipient,
            state_root: header.state_root,
            receipts_root: header.receipts_root,
            logs_bloom: header.logs_bloom,
            prev_randao: header.prev_randao,
            block_number: header.block_number,
            gas_limit: header.gas_limit,
            gas_used: header.gas_used,
            timestamp: header.timestamp,
            extra_data: header.extra_data,
            base_fee_per_gas: header.base_fee_per_gas,
            block_hash: header.block_hash,
            transactions_root: header.transactions_root,
            withdrawals_root: header.withdrawals_root,
        }
    }
}

impl From<ExecutionPayloadHeaderDeneb> for helios_types::ExecutionPayloadHeaderDeneb {
    fn from(header: ExecutionPayloadHeaderDeneb) -> Self {
        helios_types::ExecutionPayloadHeaderDeneb {
            parent_hash: header.parent_hash,
            fee_recipient: header.fee_recipient,
            state_root: header.state_root,
            receipts_root: header.receipts_root,
            logs_bloom: header.logs_bloom,
            prev_randao: header.prev_randao,
            block_number: header.block_number,
            gas_limit: header.gas_limit,
            gas_used: header.gas_used,
            timestamp: header.timestamp,
            extra_data: header.extra_data,
            base_fee_per_gas: header.base_fee_per_gas,
            block_hash: header.block_hash,
            transactions_root: header.transactions_root,
            withdrawals_root: header.withdrawals_root,
            blob_gas_used: header.blob_gas_used,
            excess_blob_gas: header.excess_blob_gas,
        }
    }
}

impl From<helios_types::ExecutionPayloadHeaderBellatrix> for ExecutionPayloadHeaderBellatrix {
    fn from(src: helios_types::ExecutionPayloadHeaderBellatrix) -> Self {
        Self {
            parent_hash: src.parent_hash,
            fee_recipient: src.fee_recipient,
            state_root: src.state_root,
            receipts_root: src.receipts_root,
            logs_bloom: src.logs_bloom,
            prev_randao: src.prev_randao,
            block_number: src.block_number,
            gas_limit: src.gas_limit,
            gas_used: src.gas_used,
            timestamp: src.timestamp,
            extra_data: src.extra_data,
            base_fee_per_gas: src.base_fee_per_gas,
            block_hash: src.block_hash,
            transactions_root: src.transactions_root,
        }
    }
}

impl From<helios_types::ExecutionPayloadHeaderCapella> for ExecutionPayloadHeaderCapella {
    fn from(src: helios_types::ExecutionPayloadHeaderCapella) -> Self {
        Self {
            parent_hash: src.parent_hash,
            fee_recipient: src.fee_recipient,
            state_root: src.state_root,
            receipts_root: src.receipts_root,
            logs_bloom: src.logs_bloom,
            prev_randao: src.prev_randao,
            block_number: src.block_number,
            gas_limit: src.gas_limit,
            gas_used: src.gas_used,
            timestamp: src.timestamp,
            extra_data: src.extra_data,
            base_fee_per_gas: src.base_fee_per_gas,
            block_hash: src.block_hash,
            transactions_root: src.transactions_root,
            withdrawals_root: src.withdrawals_root,
        }
    }
}

impl From<helios_types::ExecutionPayloadHeaderDeneb> for ExecutionPayloadHeaderDeneb {
    fn from(src: helios_types::ExecutionPayloadHeaderDeneb) -> Self {
        Self {
            parent_hash: src.parent_hash,
            fee_recipient: src.fee_recipient,
            state_root: src.state_root,
            receipts_root: src.receipts_root,
            logs_bloom: src.logs_bloom,
            prev_randao: src.prev_randao,
            block_number: src.block_number,
            gas_limit: src.gas_limit,
            gas_used: src.gas_used,
            timestamp: src.timestamp,
            extra_data: src.extra_data,
            base_fee_per_gas: src.base_fee_per_gas,
            block_hash: src.block_hash,
            transactions_root: src.transactions_root,
            withdrawals_root: src.withdrawals_root,
            blob_gas_used: src.blob_gas_used,
            excess_blob_gas: src.excess_blob_gas,
        }
    }
}

impl From<ExecutionPayloadHeader> for helios_types::ExecutionPayloadHeader {
    fn from(header: ExecutionPayloadHeader) -> Self {
        match header {
            ExecutionPayloadHeader::Bellatrix(b) => {
                helios_types::ExecutionPayloadHeader::Bellatrix(b.into())
            }
            ExecutionPayloadHeader::Capella(c) => {
                helios_types::ExecutionPayloadHeader::Capella(c.into())
            }
            ExecutionPayloadHeader::Deneb(d) => {
                helios_types::ExecutionPayloadHeader::Deneb(d.into())
            }
        }
    }
}

impl From<helios_types::ExecutionPayloadHeader> for ExecutionPayloadHeader {
    fn from(header: helios_types::ExecutionPayloadHeader) -> Self {
        match header {
            helios_types::ExecutionPayloadHeader::Bellatrix(b) => {
                ExecutionPayloadHeader::Bellatrix(b.into())
            }
            helios_types::ExecutionPayloadHeader::Capella(c) => {
                ExecutionPayloadHeader::Capella(c.into())
            }
            helios_types::ExecutionPayloadHeader::Deneb(d) => {
                ExecutionPayloadHeader::Deneb(d.into())
            }
        }
    }
}

#[superstruct(
    variants(Bellatrix, Capella, Deneb),
    variant_attributes(
        derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq),
        serde(deny_unknown_fields),
    )
)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightClientHeader {
    pub beacon: BeaconBlockHeader,
    #[superstruct(only(Capella, Deneb))]
    pub execution: ExecutionPayloadHeader,
    #[superstruct(only(Capella, Deneb))]
    pub execution_branch: FixedVector<B256, typenum::U4>,
}

impl Default for LightClientHeader {
    fn default() -> Self {
        LightClientHeader::Bellatrix(LightClientHeaderBellatrix::default())
    }
}

impl From<LightClientHeaderBellatrix> for helios_types::LightClientHeaderBellatrix {
    fn from(header: LightClientHeaderBellatrix) -> Self {
        helios_types::LightClientHeaderBellatrix {
            beacon: header.beacon,
        }
    }
}

impl From<LightClientHeaderCapella> for helios_types::LightClientHeaderCapella {
    fn from(header: LightClientHeaderCapella) -> Self {
        helios_types::LightClientHeaderCapella {
            beacon: header.beacon,
            execution: header.execution.into(),
            execution_branch: header.execution_branch,
        }
    }
}

impl From<LightClientHeaderDeneb> for helios_types::LightClientHeaderDeneb {
    fn from(header: LightClientHeaderDeneb) -> Self {
        helios_types::LightClientHeaderDeneb {
            beacon: header.beacon,
            execution: header.execution.into(),
            execution_branch: header.execution_branch,
        }
    }
}

impl From<helios_types::LightClientHeaderDeneb> for LightClientHeaderDeneb {
    fn from(header: helios_types::LightClientHeaderDeneb) -> Self {
        Self {
            beacon: header.beacon,
            execution: header.execution.into(),
            execution_branch: header.execution_branch,
        }
    }
}

impl From<helios_types::LightClientHeaderCapella> for LightClientHeaderCapella {
    fn from(header: helios_types::LightClientHeaderCapella) -> Self {
        Self {
            beacon: header.beacon,
            execution: header.execution.into(),
            execution_branch: header.execution_branch,
        }
    }
}

impl From<helios_types::LightClientHeaderBellatrix> for LightClientHeaderBellatrix {
    fn from(header: helios_types::LightClientHeaderBellatrix) -> Self {
        Self {
            beacon: header.beacon,
        }
    }
}

impl From<LightClientHeader> for helios_types::LightClientHeader {
    fn from(header: LightClientHeader) -> Self {
        match header {
            LightClientHeader::Bellatrix(b) => helios_types::LightClientHeader::Bellatrix(b.into()),
            LightClientHeader::Capella(c) => helios_types::LightClientHeader::Capella(c.into()),
            LightClientHeader::Deneb(d) => helios_types::LightClientHeader::Deneb(d.into()),
        }
    }
}

impl From<helios_types::LightClientHeader> for LightClientHeader {
    fn from(header: helios_types::LightClientHeader) -> Self {
        match header {
            helios_types::LightClientHeader::Bellatrix(b) => LightClientHeader::Bellatrix(b.into()),
            helios_types::LightClientHeader::Capella(c) => LightClientHeader::Capella(c.into()),
            helios_types::LightClientHeader::Deneb(d) => LightClientHeader::Deneb(d.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound = "S: ConsensusSpec")]
pub struct Update<S: ConsensusSpec> {
    pub attested_header: LightClientHeader,
    pub next_sync_committee: SyncCommittee<S>,
    pub next_sync_committee_branch: FixedVector<B256, typenum::U5>,
    pub finalized_header: LightClientHeader,
    pub finality_branch: FixedVector<B256, typenum::U6>,
    pub sync_aggregate: SyncAggregate<S>,
    pub signature_slot: u64,
}

impl From<Update<MainnetConsensusSpec>> for helios_types::Update<MainnetConsensusSpec> {
    fn from(update: Update<MainnetConsensusSpec>) -> Self {
        helios_types::Update {
            attested_header: update.attested_header.into(),
            next_sync_committee: update.next_sync_committee,
            next_sync_committee_branch: update.next_sync_committee_branch,
            finalized_header: update.finalized_header.into(),
            finality_branch: update.finality_branch,
            sync_aggregate: update.sync_aggregate,
            signature_slot: update.signature_slot,
        }
    }
}

impl From<helios_types::Update<MainnetConsensusSpec>> for Update<MainnetConsensusSpec> {
    fn from(update: helios_types::Update<MainnetConsensusSpec>) -> Self {
        Update {
            attested_header: update.attested_header.into(),
            next_sync_committee: update.next_sync_committee,
            next_sync_committee_branch: update.next_sync_committee_branch,
            finalized_header: update.finalized_header.into(),
            finality_branch: update.finality_branch,
            sync_aggregate: update.sync_aggregate,
            signature_slot: update.signature_slot,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(bound = "S: ConsensusSpec")]
pub struct FinalityUpdate<S: ConsensusSpec> {
    pub attested_header: LightClientHeader,
    pub finalized_header: LightClientHeader,
    pub finality_branch: FixedVector<B256, typenum::U6>,
    pub sync_aggregate: SyncAggregate<S>,
    pub signature_slot: u64,
}

impl From<FinalityUpdate<MainnetConsensusSpec>>
    for helios_types::FinalityUpdate<MainnetConsensusSpec>
{
    fn from(update: FinalityUpdate<MainnetConsensusSpec>) -> Self {
        helios_types::FinalityUpdate {
            attested_header: update.attested_header.into(),
            finalized_header: update.finalized_header.into(),
            finality_branch: update.finality_branch,
            sync_aggregate: update.sync_aggregate,
            signature_slot: update.signature_slot,
        }
    }
}

impl From<helios_types::FinalityUpdate<MainnetConsensusSpec>>
    for FinalityUpdate<MainnetConsensusSpec>
{
    fn from(update: helios_types::FinalityUpdate<MainnetConsensusSpec>) -> Self {
        FinalityUpdate {
            attested_header: update.attested_header.into(),
            finalized_header: update.finalized_header.into(),
            finality_branch: update.finality_branch,
            sync_aggregate: update.sync_aggregate,
            signature_slot: update.signature_slot,
        }
    }
}
