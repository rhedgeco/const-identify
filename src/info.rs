use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    fmt::Display,
    hash::Hash,
    ops::Range,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConstGeneric {
    Bool(bool),
    Char(char),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
}

impl ConstGeneric {
    pub const fn const_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(lhs), Self::Bool(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::Char(lhs), Self::Char(rhs)) => konst::const_eq!(lhs, rhs),

            (Self::U8(lhs), Self::U8(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::U16(lhs), Self::U16(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::U32(lhs), Self::U32(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::U64(lhs), Self::U64(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::U128(lhs), Self::U128(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::USize(lhs), Self::USize(rhs)) => konst::const_eq!(lhs, rhs),

            (Self::I8(lhs), Self::I8(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::I16(lhs), Self::I16(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::I32(lhs), Self::I32(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::I64(lhs), Self::I64(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::I128(lhs), Self::I128(rhs)) => konst::const_eq!(lhs, rhs),
            (Self::ISize(lhs), Self::ISize(rhs)) => konst::const_eq!(lhs, rhs),
            _ => false,
        }
    }

    pub const fn const_cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Bool(lhs), Self::Bool(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::Char(lhs), Self::Char(rhs)) => konst::const_cmp!(lhs, rhs),

            (Self::U8(lhs), Self::U8(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::U16(lhs), Self::U16(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::U32(lhs), Self::U32(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::U64(lhs), Self::U64(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::U128(lhs), Self::U128(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::USize(lhs), Self::USize(rhs)) => konst::const_cmp!(lhs, rhs),

            (Self::I8(lhs), Self::I8(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::I16(lhs), Self::I16(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::I32(lhs), Self::I32(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::I64(lhs), Self::I64(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::I128(lhs), Self::I128(rhs)) => konst::const_cmp!(lhs, rhs),
            (Self::ISize(lhs), Self::ISize(rhs)) => konst::const_cmp!(lhs, rhs),

            (Self::Bool(_), _) => Ordering::Less,
            (Self::Char(_), other) => match other {
                Self::Bool(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            (Self::U8(_), other) => match other {
                Self::Bool(_) | Self::Char(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            (Self::U16(_), other) => match other {
                Self::Bool(_) | Self::Char(_) | Self::U8(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            (Self::U32(_), other) => match other {
                Self::Bool(_) | Self::Char(_) | Self::U8(_) | Self::U16(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            (Self::U64(_), other) => match other {
                Self::Bool(_) | Self::Char(_) | Self::U8(_) | Self::U16(_) | Self::U32(_) => {
                    Ordering::Greater
                }
                _ => Ordering::Less,
            },
            (Self::U128(_), other) => match other {
                Self::Bool(_)
                | Self::Char(_)
                | Self::U8(_)
                | Self::U16(_)
                | Self::U32(_)
                | Self::U64(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            (Self::USize(_), other) => match other {
                Self::I8(_)
                | Self::I16(_)
                | Self::I32(_)
                | Self::I64(_)
                | Self::I128(_)
                | Self::ISize(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            (Self::I8(_), other) => match other {
                Self::I16(_) | Self::I32(_) | Self::I64(_) | Self::I128(_) | Self::ISize(_) => {
                    Ordering::Less
                }
                _ => Ordering::Greater,
            },
            (Self::I16(_), other) => match other {
                Self::I32(_) | Self::I64(_) | Self::I128(_) | Self::ISize(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            (Self::I32(_), other) => match other {
                Self::I64(_) | Self::I128(_) | Self::ISize(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            (Self::I64(_), other) => match other {
                Self::I128(_) | Self::ISize(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            (Self::I128(_), other) => match other {
                Self::ISize(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            (Self::ISize(_), other) => match other {
                _ => Ordering::Greater,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeInfo<'a> {
    pub file: &'a str,
    pub line: u32,
    pub column: u32,
    pub name: &'a str,
    pub type_generics: &'a [&'a TypeInfo<'a>],
    pub const_generics: &'a [ConstGeneric],
    hash: u64,
}

impl<'a> Display for TypeInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.hash)
    }
}

impl<'a> TypeInfo<'a> {
    pub const fn const_hash(&self) -> u64 {
        self.hash
    }

    pub const fn new(
        file: &'a str,
        line: u32,
        column: u32,
        name: &'a str,
        type_generics: &'a [&'a TypeInfo<'a>],
        const_generics: &'a [ConstGeneric],
    ) -> Self {
        Self {
            file,
            line,
            column,
            name,
            type_generics,
            const_generics,
            hash: {
                let mut hash = const_fnv1a_hash::fnv1a_hash_str_64(file);
                hash ^= const_fnv1a_hash::fnv1a_hash_64(&line.to_be_bytes(), None);
                hash ^= const_fnv1a_hash::fnv1a_hash_64(&column.to_be_bytes(), None);
                hash ^= const_fnv1a_hash::fnv1a_hash_str_64(name);

                let mut i = 0;
                while i < type_generics.len() {
                    hash ^= type_generics[i].hash;
                    i += 1;
                }

                let mut i = 0;
                while i < const_generics.len() {
                    hash ^= match const_generics[i] {
                        ConstGeneric::Bool(v) => const_fnv1a_hash::fnv1a_hash_64(&[v as u8], None),
                        ConstGeneric::Char(v) => const_fnv1a_hash::fnv1a_hash_64(&[v as u8], None),
                        ConstGeneric::U8(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::U16(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::U32(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::U64(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::U128(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::USize(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::I8(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::I16(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::I32(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::I64(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::I128(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                        ConstGeneric::ISize(v) => {
                            const_fnv1a_hash::fnv1a_hash_64(&v.to_le_bytes(), None)
                        }
                    };
                    i += 1;
                }

                hash
            },
        }
    }

    pub const fn const_eq(&self, other: &Self) -> bool {
        macro_rules! return_false {
            ($expr:expr) => {
                if !($expr) {
                    return false;
                }
            };
        }

        // early return false in order of least comparison complexity
        return_false!(konst::const_eq!(self.hash, other.hash));
        return_false!(konst::const_eq!(self.line, other.line));
        return_false!(konst::const_eq!(self.column, other.column));
        return_false!(konst::const_eq!(self.file, other.file));
        return_false!(konst::const_eq!(self.name, other.name));

        // also test all generic parameters
        let mut i = 0;
        while i < self.type_generics.len() {
            return_false!(self.type_generics[i].const_eq(other.type_generics[i]));
            i += 1;
        }

        let mut i = 0;
        while i < self.const_generics.len() {
            return_false!(self.const_generics[i].const_eq(&other.const_generics[i]));
            i += 1;
        }

        true
    }

    pub const fn const_cmp(&self, other: &Self) -> Ordering {
        macro_rules! return_not_equal {
            ($expr:expr) => {
                match $expr {
                    Ordering::Equal => {}
                    ord => return ord,
                }
            };
        }

        // early return not equal in order of least comparison complexity
        return_not_equal!(konst::const_cmp!(self.hash, other.hash));
        return_not_equal!(konst::const_cmp!(self.line, other.line));
        return_not_equal!(konst::const_cmp!(self.column, other.column));
        return_not_equal!(konst::const_cmp!(self.file, other.file));
        return_not_equal!(konst::const_cmp!(self.name, other.name));

        // also test all generic parameters
        let mut i = 0;
        while i < self.type_generics.len() {
            return_not_equal!(self.type_generics[i].const_cmp(other.type_generics[i]));
            i += 1;
        }

        let mut i = 0;
        while i < self.const_generics.len() {
            return_not_equal!(self.const_generics[i].const_cmp(&other.const_generics[i]));
            i += 1;
        }

        Ordering::Equal
    }
}

pub unsafe trait ConstIdentify {
    const TYPE_INFO: &TypeInfo<'static>;
    fn type_info(&self) -> &'static TypeInfo<'static> {
        Self::TYPE_INFO
    }
}

macro_rules! simple_const_identify {
    ($($id:ident),+ $(,)?) => {
        $(
            unsafe impl ConstIdentify for $id {
                const TYPE_INFO: &TypeInfo<'static> = &TypeInfo::new(
                    file!(),
                    line!(),
                    column!(),
                    stringify!($id),
                    &[],
                    &[],
                );
            }
        )+
    };
}

macro_rules! generic_const_identify {
    (
        $(
            $id:ident<$($gen:ident),* $(,)?>
        ),+
        $(,)?
    ) => {
        $(
            unsafe impl <$($gen: ConstIdentify,)+> ConstIdentify for $id <$($gen,)+> {
                const TYPE_INFO: &TypeInfo<'static> = &TypeInfo::new(
                    file!(),
                    line!(),
                    column!(),
                    stringify!($id),
                    &[$($gen::TYPE_INFO,)+],
                    &[],
                );
            }
        )+
    };
}

simple_const_identify!(bool, char);
simple_const_identify!(str, String);
simple_const_identify!(u8, u16, u32, u64, u128, usize);
simple_const_identify!(i8, i16, i32, i64, i128, isize);

generic_const_identify!(Range<T>);
generic_const_identify!(Option<T>, Result<T, E>);
generic_const_identify!(Vec<T>, VecDeque<T>, LinkedList<T>);
generic_const_identify!(HashMap<K, V>, BTreeMap<K, V>);
generic_const_identify!(HashSet<T>, BTreeSet<T>);
generic_const_identify!(BinaryHeap<T>);

unsafe impl<T: ConstIdentify> ConstIdentify for [T] {
    const TYPE_INFO: &TypeInfo<'static> = &TypeInfo::new(
        file!(),
        line!(),
        column!(),
        stringify!([T]),
        &[T::TYPE_INFO],
        &[],
    );
}

unsafe impl<T: ConstIdentify, const SIZE: usize> ConstIdentify for [T; SIZE] {
    const TYPE_INFO: &TypeInfo<'static> = &TypeInfo::new(
        file!(),
        line!(),
        column!(),
        stringify!([T; SIZE]),
        &[T::TYPE_INFO],
        &[ConstGeneric::USize(SIZE)],
    );
}
