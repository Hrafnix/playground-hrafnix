use crate::definition::{BasicDefinition, BasicDefinitionType};
use crate::traits::TreePrint;
use serde::{Deserialize, Serialize};
use shareable_string::ShareableString;

/// Represents a basic data value in the frozen data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicFrozen {
    definition: BasicDefinition,
    value: ShareableString,
    hash: [u8; 32],
}

impl BasicFrozen {
    /// Creates a new `BasicFrozen` instance.
    pub fn new(definition: BasicDefinition) -> Self {
        let value = definition.default_value();

        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    /// Creates a new `BasicFrozen` instance.
    pub fn new_with_value(definition: BasicDefinition, value: ShareableString) -> Self {
        let mut s = Self {
            definition,
            value,
            hash: [0u8; 32],
        };
        s.update_hash();
        s
    }

    fn update_hash(&mut self) {
        let mut h = blake3::Hasher::new();

        // Domain separation for this node/type.
        h.update(&[0x01]);
        h.update(b"Basic");

        h.update(&self.value.current_blake3_hash());

        let digest = h.finalize();
        self.hash = *digest.as_bytes();
    }

    /// Returns the value as a `ShareableString`.
    pub fn value(&self) -> ShareableString {
        self.value.clone()
    }

    /// Returns a reference to the basic definition.
    pub fn definition(&self) -> &BasicDefinition {
        &self.definition
    }

    /// Returns the pre-calculated BLAKE3 hash of the value.
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }
}

impl PartialEq<&BasicFrozen> for BasicFrozen {
    fn eq(&self, other: &&BasicFrozen) -> bool {
        self == *other
    }
}

impl PartialEq<BasicFrozen> for &BasicFrozen {
    fn eq(&self, other: &BasicFrozen) -> bool {
        *self == other
    }
}

impl TreePrint for BasicFrozen {
    fn tree_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        label: &str,
        prefix: &str,
        last: bool,
    ) -> std::fmt::Result {
        let definition_type = match self.definition().type_definition() {
            BasicDefinitionType::String => "String",
            BasicDefinitionType::File(_) => "File",
            BasicDefinitionType::Number => "Number",
            BasicDefinitionType::Choice(_) => "Choice",
        };

        writeln!(
            f,
            "{}{}{} ({}) {} - \"{}\"",
            prefix,
            Self::branch_char(last),
            label,
            self.definition.description(),
            definition_type,
            self.value,
        )
    }
}
