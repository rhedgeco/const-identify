mod info;

pub use info::{ConstGeneric, ConstIdentify, TypeInfo};

// re-export derive macro
pub use const_identify_derive::ConstIdentify;
