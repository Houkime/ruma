#[cfg(feature = "rand")]
use crate::generate_localpart;

opaque_identifier! {
    /// A Matrix key ID.
    ///
    /// Device identifiers in Matrix are completely opaque character sequences. This type is
    /// provided simply for its semantic value.
    pub type DeviceId;
}

impl DeviceId {
    /// Generates a random `DeviceId`, suitable for assignment to a new device.
    #[cfg(feature = "rand")]
    pub fn new() -> Box<Self> {
        Self::from_owned(generate_localpart(8))
    }
}

#[cfg(all(test, feature = "rand"))]
mod tests {
    use super::DeviceId;

    #[test]
    fn generate_device_id() {
        assert_eq!(DeviceId::new().as_str().len(), 8);
    }
}
