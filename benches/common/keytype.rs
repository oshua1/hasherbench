use core::hash::Hash;
use std::random::DefaultRandomSource;
use std::random::Random;

const MAX_KEYS: usize = 100_000;

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
    ($idx:ident, $length:expr) => {
        paste::paste! (
            impl $crate::common::ProduceKey for [< String $length >] {
                #[allow (clippy::indexing_slicing, reason = "Guaranteed to succeed due to constant indices")]
                fn produce_key($idx: usize) -> Self {
                    static KEYS: std::sync::OnceLock<[String; MAX_KEYS]> = std::sync::OnceLock::new ();
                    [< String $length >](KEYS.get_or_init (|| prepare_random_string_keys::<$length>())[$idx].clone ()) }
            }
        );
    };
}

/// A String key type that consists of 16 random Ascii characters
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct String16(String);

/// A String key type that consists of 128 random Ascii characters
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct String128(String);

/// A String key type that consists of 1024 random Ascii characters
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct String1024(String);

/// A String key type that uses Formatter (slower) to generate key value
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StringFmtDyn(String);

impl_produce_key!(u32, idx, u32::try_from(idx).unwrap_or_default());
impl_produce_key!(u64, idx, idx as u64);
impl_produce_key!(u128, idx, idx as u128);
impl_produce_key!(usize, idx, idx);
impl_produce_key!(String, idx, idx.to_string());
impl_produce_key!(StringFmtDyn, idx, Self(format!("{idx}")));
impl_produce_key!(idx, 16);
impl_produce_key!(idx, 128);
impl_produce_key!(idx, 1024);

// Uses weak random algorithm and produces visible Ascii characters only
#[allow (clippy::print_stdout, reason = "Potentially long warmup info")]
#[allow (clippy::unwrap_used, reason = "Guaranteed to succeed")]
fn prepare_random_string_keys<const LENGTH: usize>() -> [String; MAX_KEYS] {
    println! ("\nPreparing {MAX_KEYS} random strings of length {LENGTH} each...");
    core::array::from_fn(|_| {
        (0..LENGTH).fold(String::with_capacity(LENGTH), |mut string, _| {
            string.push(
                char::from_u32(u32::from (<u8 as Random>::random(&mut DefaultRandomSource) % 0x5e) + 0x21)
                    .unwrap(),
            );
            string
        })
    })
}
