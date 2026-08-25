use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Supplies raw IDs to typed simulation identities.
///
/// Production applications may provide random or monotonic generators. Tests can
/// use [`SequentialIdGenerator`] for reproducible identities.
pub trait IdGenerator {
    /// Returns the next raw ID, or `None` when the generator is exhausted.
    fn next_raw_id(&mut self) -> Option<u128>;
}

/// A deterministic ID generator intended for tests, imports, and migrations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequentialIdGenerator {
    /// Raw value returned by the next call.
    next: Option<u128>,
}

impl SequentialIdGenerator {
    /// Creates a generator whose first returned value is `first`.
    #[must_use]
    pub const fn new(first: u128) -> Self {
        Self { next: Some(first) }
    }
}

impl IdGenerator for SequentialIdGenerator {
    fn next_raw_id(&mut self) -> Option<u128> {
        let current = self.next?;
        self.next = current.checked_add(1);
        Some(current)
    }
}

/// Implements an opaque simulation ID serialized as 32 lowercase hexadecimal digits.
macro_rules! define_id {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u128);

        impl $name {
            /// Creates an ID from its stable raw representation.
            #[must_use]
            pub const fn from_raw(raw: u128) -> Self {
                Self(raw)
            }

            /// Returns the stable raw representation.
            #[must_use]
            pub const fn as_raw(self) -> u128 {
                self.0
            }

            /// Requests a new typed ID from an injectable generator.
            pub fn generate(generator: &mut impl IdGenerator) -> Option<Self> {
                generator.next_raw_id().map(Self)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{:032x}", self.0)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let encoded = String::deserialize(deserializer)?;
                if encoded.len() != 32 {
                    return Err(serde::de::Error::custom(
                        "simulation IDs must contain exactly 32 hexadecimal digits",
                    ));
                }
                u128::from_str_radix(&encoded, 16)
                    .map(Self)
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

define_id!(
    DocumentId,
    "Stable identity of a persisted model or component document."
);
define_id!(SystemId, "Stable identity of a system within a document.");
define_id!(ComponentId, "Stable identity of a component instance.");
define_id!(PortId, "Stable identity of an explicitly persisted port.");
define_id!(ConnectionId, "Stable identity of a connection.");
define_id!(ProbeId, "Stable identity of a probe.");
define_id!(CommandId, "Stable identity of a document command.");
define_id!(RunId, "Identity of one simulation run.");

#[cfg(test)]
mod tests {
    use super::{ComponentId, DocumentId, SequentialIdGenerator};

    #[test]
    fn injected_generator_is_deterministic_across_id_types() {
        let mut generator = SequentialIdGenerator::new(41);

        assert_eq!(DocumentId::generate(&mut generator).unwrap().as_raw(), 41);
        assert_eq!(ComponentId::generate(&mut generator).unwrap().as_raw(), 42);
    }

    #[test]
    fn ids_round_trip_as_fixed_width_strings() {
        let id = DocumentId::from_raw(0x1a2b);
        let json = serde_json::to_string(&id).unwrap();

        assert_eq!(json, "\"00000000000000000000000000001a2b\"");
        assert_eq!(serde_json::from_str::<DocumentId>(&json).unwrap(), id);
    }

    #[test]
    fn exhausted_generator_does_not_repeat_ids() {
        let mut generator = SequentialIdGenerator::new(u128::MAX);

        assert!(DocumentId::generate(&mut generator).is_some());
        assert!(DocumentId::generate(&mut generator).is_none());
    }
}
