use alloy::sol;
use serde::{ser::SerializeStruct, Serialize};
use Operator::{QueuedStakeUpdate, StakeUpdateRequest};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Core,
    "abi/Core.json",
);

impl Serialize for StakeUpdateRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("StakeUpdateRequest", 3)?;
        state.serialize_field("vault", &self.vault)?;
        state.serialize_field("dss", &self.dss)?;
        state.serialize_field("toStake", &self.toStake)?;
        state.end()
    }
}

impl Serialize for QueuedStakeUpdate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("QueuedStakeUpdate", 4)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.serialize_field("startTimestamp", &self.startTimestamp)?;
        state.serialize_field("operator", &self.operator)?;
        state.serialize_field("updateRequest", &self.updateRequest)?;
        state.end()
    }
}
