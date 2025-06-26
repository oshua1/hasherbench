use core::hash::Hash;

/// Must be implemented for all types that should be used as key types for hash sets
pub trait ProduceKey: Hash + Eq + Clone + Ord {
    fn produce_key(idx: usize) -> Self;
}

macro_rules! impl_produce_key {
    ($ty:ty, $idx:ident, $expr:expr) => {
        impl $crate::common::ProduceKey for $ty {
            #[inline]
            fn produce_key($idx: usize) -> Self {
                $expr
            }
        }
    };
}

/// A String key type that prepends 8 characters to key value
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct String8(String);

/// A String key type that prepends 16 characters to key value
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct String16(String);

/// A String key type that prepends 32 characters to key value
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct String32(String);

/// A String key type that uses Formatter (slower) to generate key value
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StringFmtDyn(String);

impl_produce_key!(u32, idx, u32::try_from(idx).unwrap_or_default());
impl_produce_key!(u64, idx, idx as u64);
impl_produce_key!(u128, idx, idx as u128);
impl_produce_key!(usize, idx, idx);
impl_produce_key!(String, idx, idx.to_string());
impl_produce_key!(String8, idx, Self(prepare_string_key::<16>("01234567", idx)));
impl_produce_key!(String16, idx, Self(prepare_string_key::<24>("0123456789abcdef", idx)));
impl_produce_key!(String32, idx, Self(prepare_string_key::<24>("0123456789abcdef0123456789abcdef", idx)));
impl_produce_key!(StringFmtDyn, idx, Self(format!("{idx}")));

#[inline]
fn prepare_string_key<const SIZE: usize>(head: &str, idx: usize) -> String {
    let mut s = String::with_capacity(SIZE);
    s.push_str(head);
    s.push_str(&idx.to_string());
    s
}
