use std::fmt::{Display, Formatter, Result};

use const_fnv1a_hash::fnv1a_hash_str_64;

/// A unique identifier that can be created used in const contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstId {
    id: u64,
}

impl Display for ConstId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.id)
    }
}

impl ConstId {
    /// Builds a new id using a raw `value`.
    ///
    /// This should almost never be used. In fact if you are reading this you may
    /// have already been looking at the source code because these docs are hidden.
    #[doc(hidden)]
    pub const fn from_raw(value: u64) -> Self {
        Self { id: value }
    }

    /// Generates a new id by hashing a `unique_str`.
    pub const fn generate(unique_str: &str) -> Self {
        Self {
            id: fnv1a_hash_str_64(unique_str),
        }
    }

    /// Returns the inner raw value used in this id.
    ///
    /// This can be useful for making comparisons in the const context.
    pub const fn raw_value(&self) -> u64 {
        self.id
    }
}

/// This trait is used as to mark structs with an id available in const contexts.
///
/// # Safety
/// This trait should only be implemented by using the `#[derive]` macro
/// because it requires that a unique ID is assigned to every struct.
///
/// If you do implement this by hand, you must ensure that every impl
/// does not have any overlapping ids.This is done in the macro by using
/// `ConstId::generate(concat!(module_path!(), "::", stringify!(StructName)))`
pub unsafe trait ConstIdentify {
    const CONST_ID: ConstId;
}

#[cfg(test)]
mod tests {
    use super::*;
    use const_identify_derive::ConstIdentify;

    #[derive(ConstIdentify)]
    struct TestStruct;

    #[test]
    fn uniqueness() {
        const ID1: ConstId = ConstId::generate("unique1");
        const ID2: ConstId = ConstId::generate("unique2");
        const ID3: ConstId = ConstId::generate("unique3");
        assert_ne!(ID1, ID2);
        assert_ne!(ID2, ID3);
        assert_ne!(ID3, ID1);
    }

    #[test]
    fn derive_uniqueness() {
        #[derive(ConstIdentify)]
        struct TestStruct;

        #[derive(ConstIdentify)]
        struct TestStruct2;

        let local_id = TestStruct::CONST_ID;
        let local_id2 = TestStruct2::CONST_ID;
        assert_ne!(local_id, local_id2);

        let global_id = tests::TestStruct::CONST_ID;
        assert_ne!(local_id, global_id);
    }

    // this section is used to check that derive tests still hold even with wacky formatting
    #[rustfmt::skip]
    mod unformatted {
        use super::*;

        #[derive(ConstIdentify)]struct TestStruct;#[test]fn derive_uniqueness() {#[derive(ConstIdentify)]struct TestStruct;
            let local_id = TestStruct::CONST_ID;
            let global_id = tests::TestStruct::CONST_ID;
            assert_ne!(local_id, global_id);
        }
    }
}
