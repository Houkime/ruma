//! Types for the `m.key.verification.key` event.

use ruma_events_macros::EventContent;
use serde::{Deserialize, Serialize};

#[cfg(feature = "unstable-pre-spec")]
use super::Relation;

/// The content of a to-device `m.key.verification.key` event.
///
/// Sends the ephemeral public key for a device to the partner device.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.key.verification.key", kind = ToDevice)]
pub struct ToDeviceKeyVerificationKeyEventContent {
    /// An opaque identifier for the verification process.
    ///
    /// Must be the same as the one used for the `m.key.verification.start` message.
    pub transaction_id: String,

    /// The device's ephemeral public key, encoded as unpadded Base64.
    pub key: String,
}

impl ToDeviceKeyVerificationKeyEventContent {
    /// Creates a new `ToDeviceKeyVerificationKeyEventContent` with the given transaction ID and
    /// key.
    pub fn new(transaction_id: String, key: String) -> Self {
        Self { transaction_id, key }
    }
}

/// The content of an in-room `m.key.verification.key` event.
///
/// Sends the ephemeral public key for a device to the partner device.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg(feature = "unstable-pre-spec")]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.key.verification.key", kind = Message)]
pub struct KeyVerificationKeyEventContent {
    /// The device's ephemeral public key, encoded as unpadded Base64.
    pub key: String,

    /// Information about the related event.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Relation,
}

#[cfg(feature = "unstable-pre-spec")]
impl KeyVerificationKeyEventContent {
    /// Creates a new `KeyVerificationKeyEventContent` with the given key and relation.
    pub fn new(key: String, relates_to: Relation) -> Self {
        Self { key, relates_to }
    }
}
