use std::{cmp::Ordering, marker::PhantomData};

use const_identify::ConstIdentify;

#[derive(ConstIdentify)]
pub struct MyCoolType<T1, T2, T3, const SIZE: usize> {
    _type: PhantomData<fn(T1, T2, T3)>,
}

fn main() {
    use const_identify::{ConstIdentify, TypeInfo};
    const LHS: &TypeInfo = MyCoolType::<u16, u32, u64, 10>::TYPE_INFO;
    const RHS: &TypeInfo = MyCoolType::<u16, u32, String, 20>::TYPE_INFO;
    const ORDERING: Ordering = LHS.const_cmp(&RHS);
    println!("{LHS} [{ORDERING:?}] {RHS}");
    println!("LHS = {LHS:?}");
}
