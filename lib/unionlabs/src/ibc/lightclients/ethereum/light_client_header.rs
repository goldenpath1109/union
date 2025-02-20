use macros::model;
use ssz::Ssz;
use typenum::Unsigned;

use crate::{
    errors::{ExpectedLength, InvalidLength, MissingField},
    ethereum::config::{
        consts::{floorlog2, EXECUTION_PAYLOAD_INDEX},
        BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES,
    },
    hash::H256,
    ibc::lightclients::ethereum::{
        beacon_block_header::{BeaconBlockHeader, TryFromBeaconBlockHeaderError},
        execution_payload_header::{ExecutionPayloadHeader, TryFromExecutionPayloadHeaderError},
    },
};

#[derive(Ssz)]
#[model(proto(
    raw(protos::union::ibc::lightclients::ethereum::v1::LightClientHeader),
    into,
    from
))]
#[serde(bound(serialize = "", deserialize = ""))]
pub struct LightClientHeader<C: BYTES_PER_LOGS_BLOOM + MAX_EXTRA_DATA_BYTES> {
    pub beacon: BeaconBlockHeader,
    pub execution: ExecutionPayloadHeader<C>,
    pub execution_branch: [H256; floorlog2(EXECUTION_PAYLOAD_INDEX)],
}

impl<C: BYTES_PER_LOGS_BLOOM + MAX_EXTRA_DATA_BYTES> From<LightClientHeader<C>>
    for protos::union::ibc::lightclients::ethereum::v1::LightClientHeader
{
    fn from(value: LightClientHeader<C>) -> Self {
        Self {
            beacon: Some(value.beacon.into()),
            execution: Some(value.execution.into()),
            execution_branch: Vec::from(value.execution_branch)
                .into_iter()
                .map(H256::into_bytes)
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TryFromLightClientHeaderError {
    MissingField(MissingField),
    BeaconBlockHeader(TryFromBeaconBlockHeaderError),
    ExecutionPayloadHeader(TryFromExecutionPayloadHeaderError),
    ExecutionBranch(InvalidLength),
    ExecutionBranchNode(InvalidLength),
}

impl<C: BYTES_PER_LOGS_BLOOM + MAX_EXTRA_DATA_BYTES>
    TryFrom<protos::union::ibc::lightclients::ethereum::v1::LightClientHeader>
    for LightClientHeader<C>
{
    type Error = TryFromLightClientHeaderError;

    fn try_from(
        value: protos::union::ibc::lightclients::ethereum::v1::LightClientHeader,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            beacon: value
                .beacon
                .ok_or(TryFromLightClientHeaderError::MissingField(MissingField(
                    "beacon",
                )))?
                .try_into()
                .map_err(TryFromLightClientHeaderError::BeaconBlockHeader)?,
            execution: value
                .execution
                .ok_or(TryFromLightClientHeaderError::MissingField(MissingField(
                    "execution",
                )))?
                .try_into()
                .map_err(TryFromLightClientHeaderError::ExecutionPayloadHeader)?,
            execution_branch: value
                .execution_branch
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()
                .map_err(TryFromLightClientHeaderError::ExecutionBranchNode)?
                .try_into()
                .map_err(|vec: Vec<_>| {
                    TryFromLightClientHeaderError::ExecutionBranch(InvalidLength {
                        expected: ExpectedLength::Exact(C::MAX_EXTRA_DATA_BYTES::USIZE),
                        found: vec.len(),
                    })
                })?,
        })
    }
}
