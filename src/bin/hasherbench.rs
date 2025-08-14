//! A "simple" Rust tool to benchmark performances of various hashing algorithms and collections with various key types,
//! collection sizes and other parameters. Mostly ones optimized for speed; **not** cryptographically secure ones!
//! Useful to compare and find best solution for particular use cases. Configured by several command line arguments,
//! defined by [`Args`].
//!
//! This crate needs **Rust Nightly** compiler to build. The following unstable features are used:
//! - `hashmap_internals` - for access to experimental `SipHasher13`
//! - `random` - for access to [`core::random::Random`] and [`std::random::DefaultRandomSource`]
//!
//! # Features
//!
//! - Multiple hashing algorithms, Rust-native implementations only. Currently included:
//!     - [`Adler32`]
//!     - [`AHasher`]
//!     - [`Bricolage`]
//!     - [`CityHasher32`]
//!     - [`CityHasher64`]
//!     - [`CityHasher128`]
//!     - [`DJB2Hasher`]
//!     - [`FarmHasher`]
//!     - [`FNV1aHasher32`]
//!     - [`FNV1aHasher64`][]
//!     - [`FoldHash` (fast)][foldhash::fast::FoldHasher]
//!     - [`FoldHash` (quality)][foldhash::quality::FoldHasher]
//!     - [`FxHasher32`]
//!     - [`FxHasher64`]
//!     - [`HashHasher`]
//!     - [`HighwayHasher`]
//!     - [`IntHasher`]
//!     - [`Lookup3Hasher`]
//!     - [`MetroHash64`]
//!     - [`MetroHash128`]
//!     - [`Murmur3Hasher`]
//!     - [`Murmur3Hasher128`]
//!     - [`Murmur3Hasher128x64`]
//!     - [`OAATHasher`]
//!     - [`RapidHasher`]
//!     - [`SDBMHasher`]
//!     - [`SeaHasher`]
//!     - [`SipHasher`]
//!     - [`SipHasher13`]
//!     - [`SpookyHasher`]
//!     - [`WyHash`][wyhash::WyHash]
//!     - [`WyHash3` final][wyhash::final3::WyHash]
//!     - [`XxHash3_64`]
//!     - [`XxHash32`]
//!     - [`XxHash64`]
//!     - [`ZwoHasher`]
//! - Multiple collection types to apply. Not all use hashing algorithms, yet may perform better in some use cases:
//!     - [`StdLib` `HashSet`][std::collections::HashSet]
//!     - [Hashbrown `HashSet`][hashbrown::HashSet] - Rust port of Google's *`SwissTable`*
//!     - [`BTreeSet`] (no hashing),
//!     - [`LiteMap`] (no hashing),
//!     - [`VectorMap`][vector_map::VecMap] (no hashing),
//!     - [`VecSet`][vecmap::VecSet] (no hashing),
//!     - Dummy (no collection, just hashing)
//! - Benchmarking sets filled with any number of items between 1 and 1 million
//! - Custom key offsets and steps, to spread key ranges (like eg, 3010, 3017, 3024, 3031, 3038 etc. with `--offset=3010
//!   --step=7`
//! - Key types `u32`, `u64`, `u128` or `String`
//! - String keys with lengths between 4 and 10,000 random (printable Ascii) characters each
//! - Measuring setup or lookup operations in (Hash)Sets
//! - Measurement of (Hash)Set lookup hits and misses of arbitrary percentages
//! - Using either [`Instant`] or [`SystemTime`] as timing source; see [`TimerSourceEnum`] for rationale
//! - Output formats
//!     - plain text
//!     - CSV
//!     - JSON
//! - Human-readable or compact output formats
//! - Customizable warmup amd execution time per permutation; to balance precision vs. execution duration
//! - Either single- or multi-threaded execution of benchmarks
//!
//! # Usage
//! Invoke without arguments to see all options. Invoke **`hasherbench --help`** for more verbose help.
//! ```Benchmark several kinds of Hashers, key types and HashSets
//! Usage: hasherbench [OPTIONS] <--coll <COLLECTION>...|--hasher <HASHER>...|--offset <OFFSET>...|--keytype <KEYTYPE>...|--length <LENGTH>...|--op <OP>...|--percent <HIT_RATE>...|--size <SIZE>...|--step <STEP>...>
//!
//! Options:
//!   -h, --help     Print help (see more with '--help')
//!   -V, --version  Print version
//!
//! Permutation parameters:
//!   -c, --coll <COLLECTION>...   Type(s) of collections to benchmark [default: std] [possible values: none, std, hashbrown, btreeset, litemap, vecmap, vectormap, all]
//!   -H, --hasher <HASHER>...     Hash algorithms to benchmark [default: sip,sip13,adler32,fx64] [possible values: adler32, ahasher, bricolage, city32, city64, city128, djb2, farm, fnv1a32, fnv1a64, foldhashfast, foldhashqual, fx32, fx64, hashhasher, highway, inthasher, lookup3, metro64, metro128, murmur32, murmur128, murmur128x64, oaat, rapid, sdbm, sea, sip, sip13, spooky, wy1, wy3, xx364, xx32, xx64, zwo, all]
//!       --offset <OFFSET>...     First index number to use for generated keys [default: 1]
//!   -K, --keytype <KEYTYPE>...   Key types [default: u32,u64,u128,S] [possible values: u32, u64, u128, string, all]
//!   -l, --length <LENGTH>...     Length(s) for string key type. Valid range: `4...10_000` [default: 8,32]
//!       --op <OP>...             Operation(s) to perform [default: lookup] [possible values: setup, lookup]
//!       --percent <HIT_RATE>...  Percentage of key hit rate, i.e. how many lookups will find something [default: 100]
//!   -s, --size <SIZE>...         Number of elements to lookup in hashset. Valid range: `1...1_000_000` [default: 100,1000,10000]
//!       --step <STEP>...         Value to add to key to generate next one. Valid range: `1...1_000_000` [default: 1] [aliases: --add]
//!
//! General operation parameters:
//!   -D, --maxms <RUN_MS>         Maximum duration in milliseconds for each benchmark to run [default: 1000]
//!   -P, --threads <THREADS>      Parallelism, number of threads to use for execution [default: 1] [aliases: --parallelism]
//!   -t, --tolerance <TOLERANCE>  Maximum timing tolerance in percent; iteration durations with greater distance to average will be dropped [default: 15]
//!   -T, --timer <TIMER>          Facility to use for measuring timing [default: instant] [possible values: instant, systime]
//!   -W, --warmup <WARMUP_MS>     Warmup duration in milliseconds for each benchmark (0 = disable) [default: 100]
//!   -y, --yes                    Assume "yes" on all questions
//!
//! Output options:
//!       --output-mode <OUTPUT_MODE>  Output mode for result data: compact or human-readably formatted (see `--human-readable` for progress output) [default: formatted] [possible values: compact, formatted]
//!   -F, --format <FORMAT>...         Output data format to use [default: txt] [possible values: txt, csv, json]
//!       --human-readable             Format stdout emission of live permutation result in expanded, human-friendlier (instead of compact) form. See `--output-mode` for formatting of result data [aliases: --hr]
//!       --csv-header                 Whether a header line with field names should be emitted on CSV output
//!   -o, --file <OUTPUT_FILE>         Output file to store results at; defaults to stdout
//!   -O, --overwrite                  Overwrite existing output file if it exists (also implied by -y)
//!   -q, --quiet                      Suppress all text output apart from results in selected format(s) to stdout; abort on error or questions (unless -y). Alias for "-v quiet"
//!   -R, --reverse                    Reverse sort order of output
//!   -S, --sort <SORT>...             Sort results in output [default: time] [possible values: none, index, iterations, time, timemin, timemax, collection, hasher, keytype, size, length, offset, step, op, hitrate]
//!   -v, --verbose <VERBOSITY>        Verbosity (0 = quiet) [default: p] [possible values: quiet, err, warn, info, progress, ops, values]
//! ```
//!
//! Try executing **`hasherbench -F JSON -K u64`** for a start. Then experiment with various parameters.
//!
//! # Implementation
//! Struct [`Main`] contains most of business logic and global program state.
//!
//! ## Adding new (or removing) hashers or collection types:
//!
//! Only hashers that implement [`Hash`] trait and provide [`BuildHasher`] are supported. In ***Rust***, this is a
//! trivial requirement.
//!
//! 1. add hasher or collection crate to `Cargo.toml`
//! 1. add variant to [`HasherEnum`] or [`CollectionType`]
//! 1. add variant to `impl Display for HasherEnum` or `impl Display for CollectionType`
//! 1. add variant to [`HasherEnum::expand()`] or [`CollectionType::expand()`]
//! 1. add variant to [`Main::create_collection()`].<br> Easiest if hasher implements [`BuildHasherDefault`]. If it does
//!    not, another expression to create its [`BuildHasher`] must be given. Like for *`FoldHasher`*s.
//! 1. for collection: regard [`CollectionType::does_not_use_hasher()`]

// IDEA: GnuPlot diagram creation feature? Yet, best type of graphs and parameter sets unclear.
// IDEA: make fields to include in output selectable by command line argument

#![feature(hashmap_internals, random)]
#![allow(internal_features, deprecated, reason = "Needed or parts of this application")]
#![allow(single_use_lifetimes, reason = "impl CollectionTrait for vector_map_VecSet::get() fails without (otherwise unneeded) lifetime annotation")]
#![allow(clippy::unit_arg, reason = "More concise returning of Ok(()) results")]

extern crate alloc;

use alloc::collections::BTreeSet;
use core::cmp::Ordering;
use core::cmp::min;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::hash::BuildHasher;
use core::hash::BuildHasherDefault;
use core::hash::Hash;
use core::hash::Hasher;
use core::hash::SipHasher;
use core::hash::SipHasher13;
use core::marker::PhantomData;
use core::random::Random;
use core::sync::atomic::AtomicU8;
use core::time::Duration;
use std::fs::File;
use std::hash::RandomState;
use std::io::Write;
use std::io::stdout;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::random::DefaultRandomSource;
use std::time::Instant;
use std::time::SystemTime;

use adler2::Adler32;
use ahash::AHasher;
use chrono::DateTime;
use chrono::Local;
use clap::CommandFactory;
use clap::Parser;
use clap::ValueEnum;
use clap::value_parser;
use farmhash::FarmHasher;
use hash_hasher::HashHasher;
use hashers::fnv::FNV1aHasher32;
use hashers::fnv::FNV1aHasher64;
use hashers::fx_hash::FxHasher32;
use hashers::fx_hash::FxHasher64;
use hashers::jenkins::Lookup3Hasher;
use hashers::jenkins::OAATHasher;
use hashers::jenkins::spooky_hash::SpookyHasher;
use hashers::oz::DJB2Hasher;
use hashers::oz::SDBMHasher;
use hashers::pigeon::Bricolage;
use highhash::Murmur3Hasher;
use highhash::city::CityHasher32;
use highhash::city::CityHasher64;
use highhash::city::CityHasher128;
use highhash::murmur::Murmur3Hasher128;
use highhash::murmur::Murmur3Hasher128x64;
use highway::HighwayHasher;
use integer_hasher::IntHasher;
use litemap::LiteMap;
use metrohash::MetroHash64;
use metrohash::MetroHash128;
use parking_lot::Mutex;
use rapidhash::RapidHasher;
use seahash::SeaHasher;
use stack_dst::ValueA;
use sysinfo::CpuRefreshKind;
use sysinfo::System;
use twox_hash::XxHash3_64;
use twox_hash::XxHash32;
use twox_hash::XxHash64;
use zwohash::ZwoHasher;


/// Own [`Result`] type used throughout this application.
///
/// Uses our own Error type with comprises [`ErrorKind`] and descriptive text.
type MyResult<T> = core::result::Result<T, Error>;

/// Trait object of type [`CollectionTrait`] that is stored on stack
type Collection = ValueA<dyn CollectionTrait, [usize; 8]>;

/// Trait object of type [`Write`] that is stored on stack
type Writer = ValueA<dyn Write, [usize; 4]>;



/// Slightly simplify / reduce boilerplate for implementing standard [`core::fmt::Display`] for own enum types.
macro_rules! impl_display_for_enum {
    ($self:ident, $ty:ty, $expr:expr) => {
        impl core::fmt::Display for $ty {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let $self = self;
                f.write_str($expr)
            }
        }
    };
}

/// Allocate trait object on stack via [`stack_dst::ValueA`]. Handle allocation error.
///
/// Arguments:
/// - **`$trait`** - Trait type to create an instance of, e.g. [`dyn Write`][std::io::Write]
/// - **`$storage`** - storage to reserve for allocation, e.g. `[usize; 4]`. 1 usize is used internally by [`ValueA`].
/// - **`$create`** - expression to create instance of $trait
macro_rules! alloc_stack {
    ($trait:ty, $storage:ty, $create:expr) => {
        stack_dst::ValueA::<$trait, $storage>::new_stable($create, |v| v as &$trait)
            // Maybe better panic? This is an unrecoverable, severe software error!
            .map_err (|err| $crate::Error::new (ErrorKind::Internal,
                &format! ("Software error: insufficient storage size {} for {} specified: {err:?}", stringify!($storage), stringify!($trait))))
    };
}


/// Whether this binary was built in `Debug` or `Release` mode
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy)]
#[allow(dead_code, reason = "Only one variant of this enum may ever be used")]
enum BinaryBuildMode {
    Debug,
    Release,
}

#[rustfmt::skip] impl_display_for_enum!(this, BinaryBuildMode, match this {
    | Self::Debug   => "Debug",
    | Self::Release => "Release",
});


/// Operation to measure in benchmarks.
///
/// Setup mainly measures efficiency of collection implementation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, ValueEnum)]
#[value(rename_all = "lower")]
#[repr(u8)]
enum HashOp {
    /// Benchmark filling set with keys
    Setup,
    /// Benchmark looking up for keys in set
    Lookup,
}

#[rustfmt::skip] impl_display_for_enum!(this, HashOp, match this {
    | Self::Setup   => "setup",
    | Self::Lookup  => "lookup",
});

impl HashOp {
    fn prepare(self, permutation: &PermutationSpec, collection: &mut Collection, string_keys: &[String]) {
        match self {
            | Self::Setup => (),
            | Self::Lookup => Self::setup_collection(permutation, collection, string_keys),
        }
    }

    fn execute(self, permutation: &PermutationSpec, collection: &mut Collection, string_keys: &[String]) {
        match self {
            | Self::Setup => Self::setup_collection(permutation, collection, string_keys),
            | Self::Lookup => {
                let is_int_key = permutation.key_type.is_int_key();
                let step = if is_int_key { permutation.step as usize } else { 1 };
                let key_begin = if is_int_key { permutation.offset as usize } else { 0 };
                let key_end = key_begin + permutation.size as usize * step;
                (key_begin..key_end).step_by(step).for_each(|idx| {
                    collection.get(&permutation.key_type.produce_key(string_keys, idx));
                });
            },
        }
    }

    fn setup_collection(permutation: &PermutationSpec, collection: &mut Collection, string_keys: &[String]) {
        collection.clear();
        let is_int_key = permutation.key_type.is_int_key();
        let add = if is_int_key { permutation.step as usize } else { 1 };
        let key_begin = if is_int_key { permutation.offset as usize } else { 0 };
        let key_end = key_begin + permutation.size as usize * add;
        let modulus = add * 100;
        let threshold = add * usize::from(permutation.hit_rate_perc);
        (key_begin..key_end)
            .step_by(add)
            .filter(|idx| idx % modulus < threshold)
            .for_each(|idx| collection.insert(permutation.key_type.produce_key(string_keys, idx)));
    }
}


/// Type of *Set collection to use for benchmarking.
///
/// Only standard [`HashSet`][std::collections::HashSet] and [`hashbrown::HashSet`] use selectable hashers.
/// [`RapidHashSet`][rapidhash::RapidHashSet] always uses [`RapidHasher`]. Other collections don't use hashers at all
/// but access elements by other strategies. They are provided for comparison of some edge use cases, where they may be
/// faster than genuine `HashSets` / `HashMaps`.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, ValueEnum)]
#[value(rename_all = "lower")]
#[repr(u8)]
enum CollectionType {
    /// Do not use any collection. Just calculate raw hashing performance.
    #[value(alias = "raw")]
    None,
    /// [`std::collections::HashSet`]
    #[value(alias = "stdhashset")]
    Std,
    /// Rust port of Google's fast *`SwissTable`* set
    Hashbrown,
    /// [`BTreeSet`] doesn't use hashing, yet may be faster in some use cases than genuine
    /// `HashSets`.
    BtreeSet,
    /// [`LiteMap`] doesn't use hashing, yet may be faster in some use cases than genuine `HashSets`.
    Litemap,
    /// [`vecmap::VecSet`]. Doesn't use hashing, yet may be faster in some use cases than genuine `HashSets`.
    VecMap,
    /// [`vector_map::VecMap`]. Doesn't use hashing, yet may be faster in some use cases than genuine `HashSets`.
    VectorMap,
    /// Dummy entry. Resolved by [`CollectionType::expand()`]. Must always be last item!
    All,
}

#[rustfmt::skip] impl_display_for_enum!(this, CollectionType, match this {
    | Self::None        => "None",
    | Self::Std         => "StdHashset",
    | Self::Hashbrown   => "Hashbrown",
    | Self::BtreeSet    => "BTreeSet",
    | Self::Litemap     => "Litemap",
    | Self::VecMap      => "VecMap",
    | Self::VectorMap   => "VectorMap",
    | Self::All         => "all",
});

impl CollectionType {
    fn expand(collections: &[Self]) -> MyResult<Vec<Self>> {
        if collections.contains(&Self::All) {
            Main::assert_correct_len(Self::All as usize, "CollectionType", vec![
                Self::None,
                Self::Std,
                Self::Hashbrown,
                Self::BtreeSet,
                Self::Litemap,
                Self::VecMap,
                Self::VectorMap,
            ])
        } else {
            Ok(Main::dedup(collections))
        }
    }

    /// Whether a collection actually uses a hasher. Not all do.
    const fn does_not_use_hasher(self) -> bool { !matches!(self, Self::Std | Self::Hashbrown) }
}


/// Hashing algorithm to benchmark.
///
/// Gathered from various sources and crates. Only pure Rust-native implementations are used, no
/// wrappers for C/C++ implementations.
///
/// Focus is on performant hashers, not on cryptographically secure hashers, which are much slower.
/// Adler32 is an exemption, to use for comparison.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, ValueEnum)]
#[value(rename_all = "lower")]
#[allow(clippy::upper_case_acronyms, reason = "Some hashers have such names")]
#[repr(u8)]
enum HasherEnum {
    /// 👎 Checksum hasher, slow
    Adler32,
    AHasher,
    Bricolage,
    #[value(alias = "CityHasher32")]
    City32,
    #[value(alias = "CityHasher64")]
    City64,
    #[value(alias = "CityHasher128")]
    City128,
    #[value(alias = "DJB2Hasher")]
    DJB2,
    #[value(alias = "FarmHasher")]
    Farm,
    FNV1a32,
    FNV1a64,
    /// 👍 One of the fastest
    FoldhashFast,
    FoldhashQual,
    #[value(alias = "FxHasher32")]
    Fx32,
    /// 👍 Often fastest
    #[value(alias = "FxHasher64")]
    Fx64,
    /// 👎 Dummy hasher, slow
    HashHasher,
    Highway,
    /// 👎 Useful for index-like integer keys only, otherwise very slow
    IntHasher,
    #[value(alias = "Lookup3Hasher")]
    Lookup3,
    #[value(alias = "MetroHash64")]
    Metro64,
    #[value(alias = "MetroHash128")]
    Metro128,
    #[value(alias = "Murmur3_32")]
    Murmur32,
    #[value(alias = "Murmur3_128")]
    Murmur128,
    #[value(alias = "Murmur3_128x64")]
    Murmur128x64,
    #[value(alias = "OAATHasher")]
    OAAT,
    #[value(alias = "RapidHasher")]
    Rapid,
    #[value(alias = "SDBMHasher")]
    SDBM,
    #[value(alias = "SeaHasher")]
    Sea,
    /// Rust's standard hasher
    #[value(alias = "SipHasher")]
    Sip,
    #[value(alias = "SipHasher13")]
    Sip13,
    #[value(alias = "SpookyHasher")]
    Spooky,
    #[value(alias = "WyHash1")]
    Wy1,
    #[value(alias = "WyHash3")]
    Wy3,
    #[value(alias = "XXHash3_64")]
    XX3_64,
    #[value(alias = "XXHash32")]
    XX32,
    #[value(alias = "XXHash64")]
    XX64,
    /// 👍 One of the fastest
    #[value(alias = "ZwoHash")]
    Zwo,
    /// Dummy entry. Resolved by [`HasherEnum::expand()`]. Must always be last item!
    All,
}

#[rustfmt::skip] impl_display_for_enum! (this, HasherEnum, match this {
    | Self::Adler32         => "Adler32 (cryptographic)",
    | Self::AHasher         => "AHasher",
    | Self::Bricolage       => "Bricolage",
    | Self::City32          => "CityHasher32",
    | Self::City64          => "CityHasher64",
    | Self::City128         => "Cityhasher128",
    | Self::DJB2            => "DJB2Hasher",
    | Self::Farm            => "FarmHasher",
    | Self::FNV1a32         => "FNV1a32",
    | Self::FNV1a64         => "FNV1a64",
    | Self::FoldhashFast    => "FoldHashFast",
    | Self::FoldhashQual    => "FoldHashQuality",
    | Self::Fx32            => "Fx32",
    | Self::Fx64            => "Fx64",
    | Self::HashHasher      => "HashHasher",
    | Self::Highway         => "Highway",
    | Self::IntHasher       => "IntHasher",
    | Self::Lookup3         => "Lookup3Hasher",
    | Self::Metro64         => "MetroHash64",
    | Self::Metro128        => "MetroHash128",
    | Self::Murmur32        => "Murmur3_32",
    | Self::Murmur128       => "Murmur3_128",
    | Self::Murmur128x64    => "Murmur3_128x64",
    | Self::OAAT            => "OAATHasher",
    | Self::Rapid           => "RapidHasher",
    | Self::SDBM            => "SDBMHasher",
    | Self::Sea             => "SeaHasher",
    | Self::Sip             => "SipHasher",
    | Self::Sip13           => "SipHasher13",
    | Self::Spooky          => "SpookyHasher",
    | Self::Wy1             => "WyHash1",
    | Self::Wy3             => "WyHashFinal3",
    | Self::XX3_64          => "XXHash3_64",
    | Self::XX32            => "XXHash32",
    | Self::XX64            => "XXHash64",
    | Self::Zwo             => "ZwoHash",
    | Self::All             => "All",
});

impl HasherEnum {
    fn expand(hashers: &[Self]) -> MyResult<Vec<Self>> {
        if hashers.contains(&Self::All) {
            Main::assert_correct_len(Self::All as usize, "Hashers", vec![
                Self::Adler32,
                Self::AHasher,
                Self::Bricolage,
                Self::City32,
                Self::City64,
                Self::City128,
                Self::DJB2,
                Self::Farm,
                Self::FNV1a32,
                Self::FNV1a64,
                Self::FoldhashFast,
                Self::FoldhashQual,
                Self::Fx32,
                Self::Fx64,
                Self::HashHasher,
                Self::Highway,
                Self::IntHasher,
                Self::Lookup3,
                Self::Metro64,
                Self::Metro128,
                Self::Murmur32,
                Self::Murmur128,
                Self::Murmur128x64,
                Self::OAAT,
                Self::Rapid,
                Self::SDBM,
                Self::Sea,
                Self::Sip,
                Self::Sip13,
                Self::Spooky,
                Self::Wy1,
                Self::Wy3,
                Self::XX3_64,
                Self::XX32,
                Self::XX64,
                Self::Zwo,
            ])
        } else {
            Ok(Main::dedup(hashers))
        }
    }
}


/// Key type to use for hashing.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, ValueEnum)]
#[value(rename_all = "lower")]
#[repr(u8)]
enum KeyType {
    U32,
    // TODO: make this alias conditional, to also support 32 bit target platforms
    #[value(alias = "usize")]
    U64,
    U128,
    /// String of -l length, filled with random, printable 7-bit ascii characters
    #[value(aliases = ["s","str", "S", "String"])]
    String,
    /// Dummy entry. Resolved by [`expand()`][KeyType::expand()]. Must always be last item!
    All,
}

#[rustfmt::skip] impl_display_for_enum!(this, KeyType, match this {
    | Self::U32     => "Int(32)",
    | Self::U64     => "Int(64)",
    | Self::U128    => "Int(128)",
    | Self::String  => "String",
    | Self::All     => "All",
});

impl KeyType {
    fn expand(keytypes: &[Self]) -> MyResult<Vec<Self>> {
        if keytypes.contains(&Self::All) {
            Main::assert_correct_len(Self::All as usize, "key types", vec![Self::U32, Self::U64, Self::U128, Self::String])
        } else {
            Ok(Main::dedup(keytypes))
        }
    }

    /// Whether selected key type is an integer key type. False = String key
    fn is_int_key(self) -> bool { matches!(self, Self::U32 | Self::U64 | Self::U128) }

    fn produce_key(self, prepared_keys: &[String], idx: usize) -> KeyVal { KeyVal::produce_key(self, prepared_keys, idx) }
}

/// Timer source to use for timing measurement. Either Instant or Systime.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, ValueEnum)]
#[value(rename_all = "lower")]
#[repr(u8)]
enum TimerSourceEnum {
    /// Use stdlib [`Instant`] as timing source.
    ///
    /// - guaranteed not to fail, i.e. always counts forwards, barring platform bugs
    /// - not guaranteed to be steady, i.e. some seconds may be longer than others
    /// - potentially higher nanoseconds precision
    Instant,
    /// Use stdlib [`SystemTime`] as timing source.
    ///
    /// - not guaranteed not to fail, i.e. 2nd query may yield time before 1st one
    /// - guaranteed to be steady, i.e. all seconds are equally long
    /// - potentially lower nanoseconds precision
    SysTime,
}


/// Format to output results to. Plain text, CSV or JSON.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, ValueEnum)]
#[value(rename_all = "lower")]
#[repr(u8)]
enum OutputFormat {
    /// Plain text
    #[value(alias = "text")]
    Txt,
    /// Comma separated values, with or without header line
    Csv,
    /// JSON data
    Json,
}

#[rustfmt::skip] impl_display_for_enum! (this, OutputFormat, match this {
    | Self::Txt     => "Plain Text",
    | Self::Csv     => "CSV",
    | Self::Json    => "JSON",
});

impl OutputFormat {
    /// Emit header to output target.
    fn write_header(self, writer: &mut dyn Write, descr: &OperationsDescription, csv_header: bool, ofm: OutputFormatMode) -> MyResult<()> {
        match self {
            | Self::Txt => Ok::<_, Error>(()),
            | Self::Csv => {
                match (csv_header, ofm) {
                    | (true, OutputFormatMode::Compact) => {
                        BenchmarkResult::iter_fields().try_for_each(|field| write!(writer, "{field},")).and_then(|()| writer.write(b"\n"))
                    },
                    | (true, OutputFormatMode::Formatted) => BenchmarkResult::iter_fields()
                        .try_for_each(|field| {
                            let field = Self::pad(field.name, field.width);
                            write!(writer, "{field},")
                        })
                        .and_then(|()| writer.write(b"\n")),
                    | (false, _) => Ok(0),
                }?;
                Ok(())
            },
            | Self::Json => {
                let (prologue, epilogue) = match ofm {
                    | OutputFormatMode::Compact => ("{\"summary\":{", "},\"results\":["),
                    | OutputFormatMode::Formatted => ("{\n  \"summary\": {\n", "\n  },\n  \"results\": [\n"),
                };
                writer.write_all(prologue.as_bytes())?;
                OperationsDescription::iter_key_value_pairs()
                    .filter(|(_key, value_fn)| !value_fn(descr).is_empty())
                    .enumerate()
                    .try_for_each(|(idx, (key, value_fn))| Self::write_json_key_val(writer, key, &value_fn(descr), 2, -26, ofm, idx == 0))?;
                writer.write_all(epilogue.as_bytes())?;
                Ok(())
            },
        }?;
        Ok(())
    }

    /// Emit footer (conclusion) to output target.
    #[rustfmt::skip]
    fn write_footer(self, writer: &mut dyn Write, ofm: OutputFormatMode) -> MyResult<()> {
        match (self, ofm) {
            | (Self::Txt | Self::Csv, _)                => Ok(0),
            | (Self::Json, OutputFormatMode::Compact)   => writer.write(b"]}\n"),
            | (Self::Json, OutputFormatMode::Formatted) => writer.write(b"\n  ]\n}\n"),
        }?;
        Ok(())
    }

    /// Write data of one [`BenchmarkResult`] in this output format into given target [`Writer`][std::io::Write].
    fn write_benchmark_result(self, writer: &mut dyn Write, benchmark_result: &BenchmarkResult, ofm: OutputFormatMode, is_first: bool) -> std::io::Result<()> {
        match self {
            | Self::Txt => match ofm {
                | OutputFormatMode::Compact => write!(writer, "{benchmark_result:?}"),
                | OutputFormatMode::Formatted => writeln!(writer, "{benchmark_result:#?}"),
            },
            | Self::Csv => BenchmarkResult::iter_fields()
                .try_for_each(|field| {
                    let value = (field.value_fn)(benchmark_result);
                    let value = if ofm == OutputFormatMode::Formatted { Self::pad(&value, field.width) } else { value };
                    write!(writer, "{value},")
                })
                .and_then(|()| writeln!(writer)),
            | Self::Json => {
                writer.write_all(if is_first { b"" } else { b",\n" })?;
                writer.write_all(if ofm == OutputFormatMode::Formatted { b"    {\n" } else { b"{" })?;
                BenchmarkResult::iter_fields().enumerate().try_for_each(|(idx, field)| {
                    let value = (field.value_fn)(benchmark_result);
                    Self::write_json_key_val(writer, field.name, &value, 3, -20, ofm, idx == 0)
                })?;
                writer.write_all(if ofm == OutputFormatMode::Formatted { b"\n    }" } else { b"}" })
            },
        }
    }

    /// Get default filename suffix of format.
    const fn filename_suffix(&self) -> &str {
        match self {
            | Self::Txt => ".txt",
            | Self::Csv => ".csv",
            | Self::Json => ".json",
        }
    }

    /// Create padded version of a `&str`
    #[allow(clippy::cast_sign_loss, clippy::cast_abs_to_unsigned, reason = "Always casting positive i8 value to usize")]
    fn pad(str: &str, min_width: i8) -> String {
        let width_abs = min_width.abs() as usize;
        if str.len() >= width_abs {
            str.to_owned()
        } else {
            let spaces = &" ".repeat(width_abs - str.len());
            let mut string = String::with_capacity(width_abs);
            if min_width < 0 {
                string.push_str(str);
                string.push_str(spaces);
            } else {
                string.push_str(spaces);
                string.push_str(str);
            }
            string
        }
    }

    /// Convert slice to JSON string representation.
    fn slice_to_json_string<T: Display>(slice: &[T]) -> String {
        let mut string = slice
            .iter()
            .fold(Vec::with_capacity(slice.len()), |mut vec, elem| {
                vec.push(elem.to_string().parse::<f32>().map_or_else(|_| format!("\"{elem}\""), |v| v.to_string()));
                vec
            })
            .join(",");
        string.insert(0, '[');
        string.push(']');
        string
    }

    /// Emit key-value pair in given output format, with or without formatting
    #[allow(clippy::cast_sign_loss, clippy::cast_abs_to_unsigned, reason = "Interested in absolute value only")]
    fn write_json_key_val(writer: &mut dyn Write, key: &str, val: &str, level: u8, min_width: i8, ofm: OutputFormatMode, first: bool) -> std::io::Result<()> {
        let value = if Self::is_numeric(val) || Self::is_array(val) { val.to_owned() } else { format!("\"{val}\"") };
        let comma = match (ofm, first) {
            | (OutputFormatMode::Compact, false) => ",",
            | (OutputFormatMode::Formatted, false) => ",\n",
            | (_, true) => "",
        };
        let (line_padding, key_padding) = match ofm {
            | OutputFormatMode::Compact => (String::new(), String::new()),
            | OutputFormatMode::Formatted => (" ".repeat((level as usize) << 1), " ".repeat((min_width.abs() as usize).saturating_sub(key.len()))),
        };
        write!(writer, "{comma}{line_padding}\"{key}\":{key_padding}{value}")
    }

    /// Simple check whether a string comprises a parseable integer or floating point value
    fn is_numeric(str: &str) -> bool { str.parse::<f32>().is_ok() }

    /// Simple check whether a string comprises a JSON-style array
    fn is_array(str: &str) -> bool { str.starts_with('[') && str.ends_with(']') }
}


/// Whether to output results in compact or formatted form
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, ValueEnum)]
#[value(rename_all = "lower")]
#[repr(u8)]
enum OutputFormatMode {
    #[value(alias = "aligned")]
    /// No padding, and in JSON: essentially single-line output
    Compact,
    /// Padding, and in JSON: one key-value pair per line
    Formatted,
}

#[rustfmt::skip] impl_display_for_enum! (this, OutputFormatMode, match this {
    | Self::Compact     => "compact",
    | Self::Formatted   => "formatted",
});


/// Sort order keys.
///
/// This is used for:
/// 1. order of executing benchmark permutations; specified by order of command line arguments,
/// 2. order of generated output data; specified by `--sort` command line argument.
///
/// Only on [`SortBy::None`] the results are written progressively as the benchmarks are going on.
/// Using any other sort order requires complete benchmark set to finish before writing results.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Copy, ValueEnum)]
#[value(rename_all = "lower")]
#[repr(u8)]
enum SortBy {
    None,
    Index,
    Iterations,
    #[value(aliases = ["timeavg", "time_avg"])]
    Time,
    #[value(alias = "time_min")]
    TimeMin,
    #[value(alias = "time_max")]
    TimeMax,
    Collection,
    Hasher,
    KeyType,
    Size,
    Length,
    Offset,
    Step,
    Op,
    HitRate,
}

#[rustfmt::skip] impl_display_for_enum! (this, SortBy, match this {
    | Self::None        => "none",
    | Self::Index       => "Index",
    | Self::Time        => "Time (avg)",
    | Self::TimeMin     => "Time (min)",
    | Self::TimeMax     => "Time (max)",
    | Self::Iterations  => "Iterations",
    | Self::Collection  => "Collection",
    | Self::Hasher      => "Hasher",
    | Self::KeyType     => "Key type",
    | Self::Size        => "Size",
    | Self::Length      => "String length",
    | Self::Offset      => "Offset",
    | Self::Step        => "Step",
    | Self::Op          => "Operation",
    | Self::HitRate     => "HitRate",
});

impl SortBy {
    /// Obtain argument name related to sort order.
    ///
    /// Values **must** match names of arguments as defined in [`ArgsPermutation`]. Sort keys not mapped to permutation
    /// selection arguments yield `None`.
    #[rustfmt::skip]
    const fn arg_name(&self) -> Option<&str> {
        match self {
            | Self::None
            | Self::Index
            | Self::Time
            | Self::TimeMin
            | Self::TimeMax
            | Self::Iterations  => None,
            | Self::Collection  => Some("collection"),
            | Self::Hasher      => Some("hasher"),
            | Self::KeyType     => Some("keytype"),
            | Self::Size        => Some("size"),
            | Self::Length      => Some("length"),
            | Self::Offset      => Some("offset"),
            | Self::Step        => Some("step"),
            | Self::Op          => Some("op"),
            | Self::HitRate     => Some("hit_rate"),
        }
    }
}


/// Verbosity levels for text messages written to stdout.
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, ValueEnum)]
#[value(rename_all = "lower")]
#[repr(u8)]
enum Verbosity {
    #[value(aliases = ["q", "0"])]
    /// Used to disable all output to stdout, abort on error or question.
    Quiet,

    #[value(aliases = ["e", "1"])]
    Err,
    #[value(aliases = ["w", "2"])]
    Warn,
    #[value(aliases = ["i", "3"])]
    Info,
    #[value(aliases = ["p", "4"])]
    Progress,
    #[value(aliases = ["o", "5"])]
    Ops,
    #[value(aliases = ["v", "6"])]
    Values,
}

#[rustfmt::skip] impl_display_for_enum! (this, Verbosity, match this {
    | Self::Quiet   => "",
    | Self::Err     => "ERROR",
    | Self::Warn    => "warn",
    | Self::Info    => "info",
    | Self::Progress=> "progress",
    | Self::Ops     => "ops",
    | Self::Values  => "values",
});


/// Kind of an error
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(u8)]
enum ErrorKind {
    /// No error
    Ok,
    /// Invalid (combination of) command line arguments were passed.
    Args,
    /// An internal or system error occurred, like failing to build thread pool.
    Internal,
    /// An I/O error of some kind.
    Io,
    // Something went wrong during execution of benchmarks
    // Exec,
}

#[rustfmt::skip] impl_display_for_enum! (this, ErrorKind, match this {
    | Self::Ok          => "OK",
    | Self::Args        => "Arguments",
    | Self::Internal    => "Internal",
    | Self::Io          => "Input / Output",
});

impl From<ErrorKind> for ExitCode {
    fn from(value: ErrorKind) -> Self { Self::from(value as u8) }
}

impl From<ErrorKind> for i32 {
    fn from(value: ErrorKind) -> Self { value as i32 }
}


/// Error type used throughout this application.
#[derive(Debug, Clone, PartialEq)]
struct Error {
    kind: ErrorKind,
    text: String,
}

impl From<Error> for ExitCode {
    fn from(value: Error) -> Self { value.kind.into() }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self { Error::new(ErrorKind::Io, &format!("{value:?}")) }
}

impl From<core::fmt::Error> for Error {
    fn from(value: core::fmt::Error) -> Self { Error::new(ErrorKind::Internal, &format!("{value:?}")) }
}

impl Error {
    fn new(kind: ErrorKind, text: &str) -> Self { Self { kind, text: text.to_owned() } }
}


/// All supported command line (CLI) arguments. Handled by [`Clap`][clap] crate.
#[derive(Debug, PartialEq, Clone, Parser)]
#[command(version, author = "oshua", about, long_about, help_expected = true, arg_required_else_help = true)]
struct Args {
    /// Arguments to specify benchmark permutations selection
    #[command(flatten, next_help_heading = "Permutation parameters")]
    permutation_spec: ArgsPermutation,
    /// General operation parameters
    #[command(flatten, next_help_heading = "General operation parameters")]
    general:          ArgsOps,
    /// Output options
    #[command(flatten, next_help_heading = "Output options")]
    output:           ArgsOutput,
}

/// CLI arguments to specify benchmark permutation. (Hasher, collection, sizes etc.)
///
/// All fields of type `Vec<_>` allow multiple values to be specified. Separated by '`,`'.
///
/// Field names **must** be in sync with output of [`SortBy::arg_name()`]!
#[derive(Debug, PartialEq, Clone, clap::Args)]
#[group(id = "Benchmark selection", required = true)]
struct ArgsPermutation {
    /// Type(s) of collections to benchmark
    #[arg (short = 'c', long = "coll", num_args = 1.., value_delimiter = ',', hide_possible_values = false, default_value = "std")]
    collection: Vec<CollectionType>,
    /// Hash algorithms to benchmark
    #[arg (short = 'H', long = "hasher", num_args = 1.., value_delimiter = ',', hide_possible_values = false, default_value = "sip,sip13,adler32,fx64", )]
    hasher:     Vec<HasherEnum>,
    /// First index number to use for generated keys
    #[arg (long = "offset", num_args = 1..=4, value_delimiter = ',', default_value = "1")]
    offset:     Vec<u32>,
    /// Key types
    #[arg (short = 'K', long = "keytype", num_args = 1..=4, value_delimiter = ',', hide_possible_values = false, default_value = "u32,u64,u128,S", )]
    keytype:    Vec<KeyType>,
    /// Length(s) for string key type. Valid range: `4...10_000`
    #[arg (short = 'l', long = "length", num_args = 1..=8, value_delimiter = ',', value_parser = value_parser!(u16).range(4..10001), hide_possible_values = false, default_value = "8,32", )]
    length:     Vec<u16>,
    /// Operation(s) to perform
    #[arg (long = "op", num_args = 1..3, value_delimiter = ',', hide_possible_values = false, default_value = "lookup", )]
    op:         Vec<HashOp>,
    /// Percentage of key hit rate, i.e. how many lookups will find something
    #[arg (long = "percent", num_args = 1..=4, value_delimiter = ',', value_parser = value_parser!(u8).range(0..101), default_value = "100")]
    hit_rate:   Vec<u8>,
    /// Number of elements to lookup in hashset. Valid range: `1...1_000_000`
    ///
    /// `offset + (add * size)` must not exceed 4 billion!
    #[arg (short = 's', long = "size", num_args = 1..11, value_delimiter = ',', value_parser = value_parser!(u32).range(1..1_000_001), default_value = "100,1000,10000", )]
    size:       Vec<u32>,
    /// Value to add to key to generate next one. Valid range: `1...1_000_000`
    ///
    /// `offset + (add * size)` must not exceed 4 billion!
    #[arg (long = "step", visible_alias = "add", num_args = 1..=4, value_delimiter = ',', value_parser = value_parser!(u32).range(1..1_000_001), default_value = "1")]
    step:       Vec<u32>,
}

/// CLI arguments to specify general operation parameters
#[derive(Debug, PartialEq, Clone, clap::Args)]
#[group(id = "General operation parameters", required = false)]
struct ArgsOps {
    /// Maximum duration in milliseconds for each benchmark to run
    #[arg (short = 'D', long = "maxms", value_parser = value_parser!(u16), default_value_t = 1000_u16)]
    run_ms:    u16,
    /// Parallelism, number of threads to use for execution
    #[arg(short = 'P', long = "threads", visible_alias = "parallelism", default_value_t = 1)]
    threads:   u8,
    /// Maximum timing tolerance in percent; iteration durations with greater distance to average will be dropped.
    ///
    /// Used by [`post_process_results()`][BenchmarkResult::create_from_raw_data].
    #[arg (short = 't', long = "tolerance", hide_possible_values = false, value_parser = value_parser!(u8).range(1..), default_value_t = 15, )]
    tolerance: u8,
    /// Facility to use for measuring timing
    #[arg(short = 'T', long = "timer", hide_possible_values = false, default_value = "instant")]
    timer:     TimerSourceEnum,
    /// Warmup duration in milliseconds for each benchmark (0 = disable)
    #[arg (short = 'W', long = "warmup", value_parser = value_parser!(u16), default_value_t = 100)]
    warmup_ms: u16,
    /// Assume "yes" on all questions
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    yes:       bool,
}

/// CLI arguments to specify results output
#[derive(Debug, PartialEq, Clone, clap::Args)]
#[group(id = "Output options", required = false)]
#[allow(clippy::struct_excessive_bools, reason = "Several switches supported; they are bool")]
// IDEA: provide field selection option
struct ArgsOutput {
    /// Output mode for result data: compact or human-readably formatted (see `--human-readable` for progress output)
    #[arg(long = "output-mode", default_value_t = OutputFormatMode::Formatted)]
    output_mode:    OutputFormatMode,
    /// Output data format to use
    #[arg (short = 'F', long = "format", num_args = 1.., value_delimiter = ',', hide_possible_values = false, default_value = "txt")]
    format:         Vec<OutputFormat>,
    /// Format stdout emission of live permutation result in expanded, human-friendlier (instead of compact) form. See
    /// `--output-mode` for formatting of result data.
    #[arg(long = "human-readable", visible_alias = "hr", default_value_t = false)]
    human_readable: bool,
    /// Whether a header line with field names should be emitted on CSV output
    #[arg(long = "csv-header", default_value_t = false)]
    csv_header:     bool,
    /// Output file to store results at; defaults to stdout
    #[arg(short = 'o', long = "file", value_parser = value_parser!(PathBuf))]
    output_file:    Option<PathBuf>,
    /// Overwrite existing output file if it exists (also implied by -y)
    #[arg(short = 'O', long = "overwrite", default_value_t = false)]
    overwrite:      bool,
    /// Suppress all text output apart from results in selected format(s) to stdout; abort on error or questions (unless
    /// -y). Alias for "-v quiet"
    #[arg(short = 'q', long = "quiet", default_value_t = false)]
    quiet:          bool,
    /// Reverse sort order for output
    #[arg(short = 'R', long = "reverse", default_value_t = false)]
    reverse_sort:   bool,
    /// Sort results in output
    #[arg (short = 'S', long = "sort", num_args = 1..4, value_delimiter = ',', hide_possible_values = false, default_value = "time", )]
    sort:           Vec<SortBy>,
    /// Verbosity (0 = quiet, 6 = very verbose, 3 = default)
    #[arg(short = 'v', long = "verbose", hide_possible_values = false, default_value = "p")]
    verbosity:      Verbosity,
}


/// Specification of one permutation, i.e. one parameter set to benchmark.
#[derive(Debug, Clone, PartialEq)]
struct PermutationSpec {
    index:         u32,
    size:          u32,
    offset:        u32,
    step:          u32,
    collection:    CollectionType,
    hasher:        HasherEnum,
    key_type:      KeyType,
    op:            HashOp,
    string_length: u16,
    hit_rate_perc: u8,
}

/// This implementation produces an invalid [`PermutationSpec`].
///
/// Used to create dummy return value when running in [`ExecutorWarmupMode::Warmup`].
impl Default for PermutationSpec {
    fn default() -> Self {
        Self {
            collection:    CollectionType::All,
            hasher:        HasherEnum::All,
            key_type:      KeyType::All,
            op:            HashOp::Lookup,
            index:         Default::default(),
            size:          Default::default(),
            offset:        Default::default(),
            step:          Default::default(),
            string_length: Default::default(),
            hit_rate_perc: Default::default(),
        }
    }
}


/// Result timings of one benchmark permutation.
///
/// Iterations are expected to last between naoseconds and milliseconds range. Maximum supported duration is 4 seconds.
/// Likely takes millions of iterations with long string data to hash to exceed this. Longer durations wouldn't add
/// substantial insights but just waste time.
#[derive(Clone, PartialEq, Default)]
struct BenchmarkResult {
    /// Permutation whose parameters created this result
    permutation:        PermutationSpec,
    /// Total number of iterations that were executed during benchmark
    iterations:         u32,
    /// Number of refinement passes to create this result
    refinements:        u32,
    /// Duration of data refinements in milliseconds
    filter_duration_ms: u32,
    /// Number of iterations that were dropped due to aberrative results
    dropped:            u32,
    /// Minimum execution time of an iteration in nanoseconds which is inside tolerance
    nanos_min:          u32,
    /// Maximum execution time of an iteration in nanoseconds which is inside tolerance
    nanos_max:          u32,
    /// Minimum execution time of an iteration in nanoseconds ever encountered, which may be outside tolerance
    nanos_min_abs:      u32,
    /// Maximum execution time of an iteration in nanoseconds ever encountered, which may be outside tolerance
    nanos_max_abs:      u32,
    /// Average execution time of an iteration in nanoseconds
    nanos_avg:          u32,
}

impl Debug for BenchmarkResult {
    #[allow(clippy::integer_division, reason = "No sub-nanosecond resolution feasible")]
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BenchmarkResult")
            .field("permutation_spec", &self.permutation)
            .field("iterations",        &self.iterations)
            .field("refinement runs",   &self.refinements)
            .field("refinement ms",     &self.filter_duration_ms)
            .field("dropped",           &self.dropped)
            .field("dropped %",         &self.dropped_percent())
            .field("max aberration >%", &((u64::from(self.nanos_max_abs) * 100 / u64::from(self.nanos_avg.max(1))).wrapping_sub(100)))
            .field("max aberration <%", &((u64::from(self.nanos_avg) * 100 / u64::from(self.nanos_min_abs.max(1))).wrapping_sub(100)))
            .field("nanos_min",         &self.nanos_min)
            .field("nanos_max",         &self.nanos_max)
            .field("nanos_min_abs",     &self.nanos_min_abs)
            .field("nanos_max_abs",     &self.nanos_max_abs)
            .field("nanos_avg",         &self.nanos_avg)
            .field("nanos_min / iter",  &self.nanos_min_per_iter())
            .field("nanos_max / iter",  &self.nanos_max_per_iter())
            .field("nanos_avg / iter",  &self.nanos_avg_per_iter())
            .finish()
    }
}


/// Description and attributes of a result field to output
#[derive(Clone, PartialEq)]
struct OutputFieldDescr {
    /// Emitted name of field; must be JSON-compatible
    name:     &'static str,
    /// Expression creating the field value from given [`BenchmarkResult`]
    value_fn: fn(&BenchmarkResult) -> String,
    /// Kind of this field / what source struct it originates from
    source:   BenchmarkResultFieldSource,
    /// Minimum width to resver for value in aligned output. Positive values = right-align, negative = left-align.
    width:    i8,
}

impl Display for OutputFieldDescr {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result { f.write_str(self.name) }
}

impl OutputFieldDescr {
    const fn new(name: &'static str, source: BenchmarkResultFieldSource, width: i8, value_fn: fn(&BenchmarkResult) -> String) -> Self {
        Self { name, value_fn, source, width }
    }
}

/// Which source struct an emitted field in result output originates from
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
#[repr(u8)]
enum BenchmarkResultFieldSource {
    // TODO: remove this type. Unneeded information.
    // GlobalParms,
    PermutationSpec,
    PermutationResult,
}

#[rustfmt::skip] impl_display_for_enum! (this, BenchmarkResultFieldSource, match this {
    // | Self::GlobalParms         => "global parameters",
    | Self::PermutationSpec     => "permutation specification",
    | Self::PermutationResult   => "permutation result",
});


#[allow(clippy::integer_division, clippy::cast_possible_truncation, reason = "Loss of precision acceptable at nanoseconds resolution")]
impl BenchmarkResult {
    /// Get iterator that returns [`OutputFieldDescr]`] of all fields to emit
    #[rustfmt::skip]
    fn iter_fields() -> core::slice::Iter<'static, OutputFieldDescr> {
        use OutputFieldDescr as OFD;
        use BenchmarkResultFieldSource as BRFS;
        static ITEMS: &[OutputFieldDescr] = &[
            OFD::new ("index",              BRFS::PermutationSpec,   4, |bmr| bmr.permutation.index.to_string()),
            OFD::new ("size",               BRFS::PermutationSpec,   6, |bmr| bmr.permutation.size.to_string()),
            OFD::new ("offset",             BRFS::PermutationSpec,   6, |bmr| bmr.permutation.offset.to_string()),
            OFD::new ("step",               BRFS::PermutationSpec,   6, |bmr| bmr.permutation.step.to_string()),
            OFD::new ("collection",         BRFS::PermutationSpec,  -20,|bmr| bmr.permutation.collection.to_string()),
            OFD::new ("hasher",             BRFS::PermutationSpec,  -24,|bmr| bmr.permutation.hasher.to_string()),
            OFD::new ("key_type",           BRFS::PermutationSpec,  -8, |bmr| bmr.permutation.key_type.to_string()),
            OFD::new ("operation",          BRFS::PermutationSpec,  -6, |bmr| bmr.permutation.op.to_string()),
            OFD::new ("string_key_length",  BRFS::PermutationSpec,   5, |bmr| bmr.permutation.string_length.to_string()),
            OFD::new ("hit_rate_percent",   BRFS::PermutationSpec,   3, |bmr| bmr.permutation.hit_rate_perc.to_string()),
            OFD::new ("iterations",         BRFS::PermutationResult, 6, |bmr| bmr.iterations.to_string()),
            OFD::new ("refinement_runs",    BRFS::PermutationResult, 2, |bmr| bmr.refinements.to_string()),
            OFD::new ("refinement_ms",      BRFS::PermutationResult, 6, |bmr| bmr.filter_duration_ms.to_string()),
            OFD::new ("dropped",            BRFS::PermutationResult, 6, |bmr| bmr.dropped.to_string()),
            OFD::new ("dropped_percent",    BRFS::PermutationResult, 4, |bmr| bmr.dropped_percent().to_string()),
            OFD::new ("max_aberration_posp",BRFS::PermutationResult, 6, |bmr| ((u64::from(bmr.nanos_max_abs) * 100 / u64::from(bmr.nanos_avg.max(1))).wrapping_sub(100)).to_string()),
            OFD::new ("max_aberration_negp",BRFS::PermutationResult, 6, |bmr| ((u64::from(bmr.nanos_avg) * 100 / u64::from(bmr.nanos_min_abs.max(1))).wrapping_sub(100)).to_string()),
            OFD::new ("nanos_min",          BRFS::PermutationResult, 8, |bmr| bmr.nanos_min.to_string()),
            OFD::new ("nanos_max",          BRFS::PermutationResult, 8, |bmr| bmr.nanos_max.to_string()),
            OFD::new ("nanos_min_abs",      BRFS::PermutationResult, 8, |bmr| bmr.nanos_min_abs.to_string()),
            OFD::new ("nanos_max_abs",      BRFS::PermutationResult, 8, |bmr| bmr.nanos_max_abs.to_string()),
            OFD::new ("nanos_avg",          BRFS::PermutationResult, 8, |bmr| bmr.nanos_avg.to_string()),
            OFD::new ("nanos_min_iter",     BRFS::PermutationResult, 4, |bmr| bmr.nanos_min_per_iter().to_string()),
            OFD::new ("nanos_max_iter",     BRFS::PermutationResult, 4, |bmr| bmr.nanos_max_per_iter().to_string()),
            OFD::new ("nanos_avg_iter",     BRFS::PermutationResult, 4, |bmr| bmr.nanos_avg_per_iter().to_string()),
        ];
        ITEMS.iter()
    }

    /// Create a `BenchmarkResult` for a benchmark by refining raw timing data.
    ///
    /// Loops through all nanosecond durations of one benchmark. Filters out all results which are lower or higher than
    /// average by [`ArgsOps::tolerance`]%. Then recalculates average and repeats process until no more aberrative
    /// elements are filtered out. Leaves at least three elements.
    #[allow(clippy::cast_precision_loss, reason = "We don't care about more than 5 digits precision of execution time")]
    fn create_from_raw_data(main: &Main, permutation: &PermutationSpec, durations_ns: &mut Vec<u64>) -> BenchmarkResult {
        let orig_len = durations_ns.len();
        let mut last_len = 0;
        let mut dropped = 0;
        let mut nanos_max_abs = 0;
        let mut nanos_min_abs = u32::MAX;
        let mut refinements = 0;
        let mut nanos_min = 0;
        let mut nanos_max = 0;
        let mut nanos_avg = 0;
        let timer = Instant::now();
        main.printmsg(
            Verbosity::Ops,
            &format!(
                "Raw result: count: {}, min: {}, avg: {}, max: {}, permutation: {permutation:?}",
                durations_ns.len(),
                *durations_ns.iter().min().unwrap_or(&u64::MAX),
                durations_ns.iter().sum::<u64>() / durations_ns.len() as u64,
                *durations_ns.iter().max().unwrap_or(&0),
            ),
        );
        while last_len != durations_ns.len() && durations_ns.len() > 3 {
            let nanos_sum;
            refinements += 1;
            last_len = durations_ns.len();
            // First implementation. Obsoleted. Can become rather slow due to multiple iterator runs.
            // nanos_min = *durations_ns.iter().min().unwrap_or(&u64::MAX) as u32;
            // nanos_avg = durations_ns.iter().sum::<u64>() / durations_ns.len() as u64;
            // nanos_max = *durations_ns.iter().max().unwrap_or(&0) as u32;
            (nanos_min, nanos_max, nanos_sum) =
                durations_ns.iter().fold((u64::MAX, u64::MIN, 0), |(min, max, sum), elem| (min.min(*elem), max.max(*elem), sum + elem));
            nanos_avg = nanos_sum / durations_ns.len() as u64;
            nanos_min_abs = u32::min(nanos_min as u32, nanos_min_abs);
            nanos_max_abs = u32::max(nanos_max as u32, nanos_max_abs);
            durations_ns.retain(|item| {
                let aberration = if *item > nanos_avg { *item * 100 / nanos_avg } else { nanos_avg * 100 / *item } - 100;
                let keep = *item != 0 && aberration <= u64::from(main.args.general.tolerance);
                // Enable this to see values processed. Produces much output and slows down processing.
                // main.printmsg(
                //     Verbosity::Values,
                //     &format!("min: {nanos_min}, avg: {nanos_avg}, max: {nanos_max}, curr: {item}, aberration: {aberration}, keep:
                // {keep}"), );
                dropped += u32::from(!keep);
                keep
            });
            main.printmsg(
                Verbosity::Values,
                &format !(
                    "Refinement iterations: {refinements}, min: {nanos_min}, avg: {nanos_avg}, max: {nanos_max}, orig_len: {orig_len} within {} ms, retained: {}, dropped: {dropped} ({}%)",
                    main.args.general.run_ms,
                    durations_ns.len(),
                    100 * dropped / orig_len as u32
                ),
            );
            if durations_ns.is_empty() {
                let permutation_string = if main.args.output.human_readable { format!("{permutation:#?}") } else { format!("{permutation:?}") };
                main.printmsg(
                    Verbosity::Err,
                    &format!(
                        "Empty durations result for {permutation_string} after {refinements} refinements. Got min: {nanos_min_abs} ns, max: {nanos_max_abs} ns. Increase timing tolerance (--tolerance) or reduce thread count."
                    ),
                );
            }
        }
        let filter_duration_ms = timer.elapsed().as_millis() as u32;
        main.printmsg(
            Verbosity::Ops,
            &format!(
                "{refinements} refinement runs on {orig_len} results took {} ms and filtered out {} ({:.1}%) results",
                filter_duration_ms,
                orig_len - durations_ns.len(),
                100.0 - (((durations_ns.len() as f32) * 100.0) / (orig_len as f32))
            ),
        );
        BenchmarkResult {
            permutation: permutation.clone(),
            iterations: durations_ns.len() as u32,
            refinements,
            filter_duration_ms,
            dropped,
            nanos_max: nanos_max as u32,
            nanos_min: nanos_min as u32,
            nanos_min_abs,
            nanos_max_abs,
            nanos_avg: nanos_avg as u32,
        }
    }

    const fn nanos_min_per_iter(&self) -> u32 { self.nanos_min / self.permutation.size }

    const fn nanos_avg_per_iter(&self) -> u32 { self.nanos_avg / self.permutation.size }

    const fn nanos_max_per_iter(&self) -> u32 { self.nanos_max / self.permutation.size }

    const fn dropped_percent(&self) -> u8 { (self.dropped * 100 / (self.iterations + self.dropped)) as u8 }
}


/// Internal state to iterate over all permutations from given arguments.
///
/// This doesn't (pre)store all permutations. It stores only list of argument values to generate permutations from.
/// Permutation is dynamically generated on call to [`next()`][PermutationsIterator::next()].
#[derive(Debug, PartialEq, Clone, Default)]
struct PermutationsIterator {
    sort_order:           Vec<SortBy>,
    sizes:                Vec<u32>,
    offsets:              Vec<u32>,
    steps:                Vec<u32>,
    collections:          Vec<CollectionType>,
    hashers:              Vec<HasherEnum>,
    keytypes:             Vec<KeyType>,
    ops:                  Vec<HashOp>,
    string_lengths:       Vec<u16>,
    hit_rates:            Vec<u8>,
    sizes_index:          usize,
    offsets_index:        usize,
    steps_index:          usize,
    collections_index:    usize,
    hashers_index:        usize,
    keytypes_index:       usize,
    ops_index:            usize,
    string_lengths_index: usize,
    hit_rates_index:      usize,
    index:                u32,
    finished:             bool,
}

impl PermutationsIterator {
    #[rustfmt::skip]
    fn new_from (perm: &ArgsPermutation, sort_order: &[SortBy]) -> Self {
        Self {
            sort_order:     Vec::from(sort_order),
            sizes:          perm.size.clone(),
            offsets:        perm.offset.clone(),
            steps:          perm.step.clone(),
            collections:    perm.collection.clone(),
            hashers:        perm.hasher.clone(),
            keytypes:       perm.keytype.clone(),
            ops:            perm.op.clone(),
            string_lengths: perm.length.clone(),
            hit_rates:      perm.hit_rate.clone(),
            ..Default::default()
        }
    }
}

impl Iterator for PermutationsIterator {
    type Item = PermutationSpec;

    /// Obtain next [`PermutationSpec`] instance to execute, according to [`ArgsPermutation`] specification.
    ///
    /// This is a rather complex iterator implementation, since it has multiple tricky properties:
    /// 1. order of results is dynamically specified by order of command line arguments given.
    /// 2. number of given values per ordering criterion is variable.
    /// 3. some key criteria ([`KeyType`], [`HasherEnum`] and `StringLength`) cannot be simply stepped further. Whether
    ///    they can, depends on other criteria:
    ///     - string length may only be stepped further if current key type is string.
    ///     - but then, key type may not be stepped further without all given string lengths being iterated first.
    ///     - some collection types don't use hashers at all; hence stepping hashers is useless for them.
    ///
    /// Fortunately this method is not performance-critical. It is not executing during benchmarking but just to fetch
    /// the next permutation specification to benchmark.
    #[allow(clippy::indexing_slicing, reason = "Validity of indices checked just before accesses")]
    fn next(&mut self) -> Option<Self::Item> {
        #[allow(clippy::indexing_slicing, reason = "Only well defined indices retrieved, and asserted that Vecs cannot be empty")]
        #[allow(clippy::panic, reason = "Only occurs on software error")]
        // #[rustfmt::skip]
        fn step(this: &mut PermutationsIterator) {
            /// Selects next value from specified argument values. If all values done, signal "overflow" by returning
            /// `Some(())`. Then next sort key will be incremented. On returning `None`, incrementing will be aborted.
            fn increment(target: &mut usize, collection_length: usize) -> Option<()> {
                *target += 1;
                *target %= collection_length;
                (*target == 0).then_some(())
            }

            /// Check whether key type can be switched to next, or string key with multiple lengths is selected, and
            /// other string key lengths need to be stepped first.
            fn can_step_key_type(this: &mut PermutationsIterator) -> bool {
                this.keytypes[this.keytypes_index] != KeyType::String ||
                    this.string_lengths.len() <= 1 ||
                    this.sort_order.iter().position(|elem| *elem == SortBy::KeyType) > this.sort_order.iter().position(|elem| *elem == SortBy::Length)
            }

            let can_step_key_type = can_step_key_type(this);
            #[rustfmt::skip]
            this.sort_order
                .iter()
                .try_for_each(|sort_key| match sort_key {
                    | SortBy::Hasher        => (this.collections[this.collections_index].does_not_use_hasher())
                        .then_some(())
                        .or_else(|| increment(&mut this.hashers_index,                      this.hashers.len())),
                    | SortBy::Length        => (this.keytypes[this.keytypes_index] != KeyType::String)
                        .then_some(())
                        .or_else(|| increment(&mut this.string_lengths_index,               this.string_lengths.len())),
                    | SortBy::KeyType       => (!can_step_key_type).then_some(()).or_else(|| increment(&mut this.keytypes_index, this.keytypes.len())),
                    | SortBy::Collection    => increment(&mut this.collections_index,   this.collections.len()),
                    | SortBy::Size          => increment(&mut this.sizes_index,         this.sizes.len()),
                    | SortBy::Offset        => increment(&mut this.offsets_index,       this.offsets.len()),
                    | SortBy::Step          => increment(&mut this.steps_index,         this.steps.len()),
                    | SortBy::Op            => increment(&mut this.ops_index,           this.ops.len()),
                    | SortBy::HitRate       => increment(&mut this.hit_rates_index,     this.hit_rates.len()),
                    | SortBy::Index
                    | SortBy::Time
                    | SortBy::TimeMin
                    | SortBy::TimeMax
                    | SortBy::Iterations
                    | SortBy::None          => panic!("Software error: unexpected sort keys found!"),
                })
                .inspect(|()| this.finished = true);
        }

        (!self.finished).then(|| {
            self.index += 1;
            let result = PermutationSpec {
                index:         self.index,
                size:          self.sizes[self.sizes_index],
                offset:        self.offsets[self.offsets_index],
                step:          self.steps[self.steps_index],
                collection:    self.collections[self.collections_index],
                hasher:        self.hashers[self.hashers_index],
                key_type:      self.keytypes[self.keytypes_index],
                op:            self.ops[self.ops_index],
                string_length: self.string_lengths[self.string_lengths_index],
                hit_rate_perc: self.hit_rates[self.hit_rates_index],
            };
            step(self);
            result
        })
    }
}

/// Iterating over Args results in all permutations, according to arguments.
impl IntoIterator for &ArgsPermutation {
    type IntoIter = PermutationsIterator;
    type Item = PermutationSpec;

    #[allow(clippy::expect_used, reason = "Determined execution sort order successfully before. So this cannot fail. And no way to forward Result<>.")]
    fn into_iter(self) -> Self::IntoIter {
        PermutationsIterator::new_from(
            self,
            Main::determine_execution_sort_order()
                .expect("Software error: execution sort order could not be determined for iterator")
                .as_slice(),
        )
    }
}


/// Key value of one of the supported key types.
///
/// This type is used as key type for all collections. Done so to avoid generic key types, which increase compile times
/// a lot. While evaluation of value in enum produces slight overhead, it's insignificant, as comparison with `cargo
/// bench` implementation approach shows, which is entirely generics-based. Also, enum shortens code boilerplate
/// substantially when compared to generic implementation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum KeyVal {
    /// A u32 key with its value
    U32(u32),
    /// A u64 key with its value
    U64(u64),
    /// A u128 key with its value
    U128(u128),
    /// A String key with random value of length specified by [`ArgsPermutation::length`].
    String(String),
}

impl Hash for KeyVal {
    #[rustfmt::skip]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            | Self::U32(v)      => state.write_u32(*v),
            | Self::U64(v)      => state.write_u64(*v),
            | Self::U128(v)     => state.write_u128(*v),
            | Self::String(v)   => state.write (v.as_bytes()),
        }
    }
}

impl KeyVal {
    #[rustfmt::skip]
    #[allow(clippy::indexing_slicing, reason = "Number of random strings exactly precalculated")]
    #[allow(clippy::cast_possible_truncation, reason = "Maximum idx size is 1M anyway")]
    #[allow(clippy::panic, reason = "Software error if reached. To preserve maximum performance, not using MyResult<> here.")]
    fn produce_key(key_type: KeyType, prepared_keys: &[String], idx: usize) -> Self {
        match key_type {
            | KeyType::U32      => Self::U32(idx as u32),
            | KeyType::U64      => Self::U64(idx as u64),
            | KeyType::U128     => Self::U128(idx as u128),
            | KeyType::String   => Self::String(prepared_keys[idx].clone()),
            | KeyType::All      => panic!("Software error: this code should never be reached! Missed expansion of key_type argument!"),
        }
    }

    #[rustfmt::skip]
    fn hash_key<BH: BuildHasher>(self, build_hasher: &BH) -> u64 {
        let mut hasher = build_hasher.build_hasher();
        match self {
            | Self::U32(v)      => hasher.write_u32(v),
            | Self::U64(v)      => hasher.write_u64(v),
            | Self::U128(v)     => hasher.write_u128(v),
            | Self::String(v)   => hasher.write(v.as_bytes()),
        }
        hasher.finish()
    }
}


/// Wrapper to eliminate generic type parameter for [`standard HashSet`][std::collections::HashSet]
#[derive(Debug, Clone)]
struct CollectionStdHashSet<BH>(std::collections::HashSet<KeyVal, BH>);
/// Wrapper to eliminate generic type parameter for [`Hashbrown HashSet`][hashbrown::HashSet]
#[derive(Debug, Clone)]
struct CollectionHashbrownHashSet<BH>(hashbrown::HashSet<KeyVal, BH>);
#[derive(Debug, Clone)]
/// Wrapper to eliminate generic type parameter for [`BTreeSet`]
struct CollectionBTreeSet<BH>(BTreeSet<KeyVal>, PhantomData<BH>);
#[derive(Debug, Clone)]
/// Wrapper to eliminate generic type parameter for [`LiteMap`]
struct CollectionLitemap<BH>(LiteMap<KeyVal, ()>, PhantomData<BH>);
/// Wrapper to eliminate generic type parameter for [`VecSet`][vecmap::VecSet]
#[derive(Debug, Clone)]
struct CollectionVecSet<BH>(vecmap::VecSet<KeyVal>, PhantomData<BH>);
/// Wrapper to eliminate generic type parameter for [`VectorMap`][vector_map::VecMap]
#[derive(Debug, Clone)]
struct CollectionVectorMap<BH>(vector_map::VecMap<KeyVal, ()>, PhantomData<BH>);
/// Dummy implementation for raw hasher benchmarking.
///
/// No [`HashSet`][std::collections::HashSet] of any type is created or queried. Only method [`get()`][Self::get()]does
/// something: a hashing operation.
#[derive(Debug, Clone)]
struct CollectionDummy<BH>(BH);


/// Uniform method to create instance of a collection (or pseudo-collection) type.
trait CollectionBuilderTrait<BH: BuildHasher+Clone>: CollectionTrait {
    /// Create new instance with given capacity
    fn with_capacity_and_hasher(capacity: usize, build_hasher: BH) -> Self;
}

/// Common methods for all supported collection and pseudo-collection types.
#[allow(dead_code, reason = "CollectionTrait.is_empty() is never used yet should be defined besides len()")]
trait CollectionTrait {
    /// (Hash and) insert one element into collection.
    fn insert(&mut self, key: KeyVal);
    /// (Hash and) query set for given key
    fn get<'a: 'b, 'b>(&'a self, key: &'b KeyVal) -> Option<&'b KeyVal>;
    /// Number of keys contained in set.
    fn len(&self) -> usize;
    /// Whether this set is empty
    fn is_empty(&self) -> bool { self.len() == 0 }
    /// Remove all elements from set.
    fn clear(&mut self);
}



/// Implement traits [`CollectionTrait`] and [`CollectionBuilderTrait`] for collection wrappers.
///
/// Allows to implement for multiple collection types by single invocation. This reduces boilerplate to implement these
/// traits substantially.
///
/// Arguments used for all implementations:
/// - **$self**: alias for `self`. To be used in expressions where `self` is needed. `self` itself cannot be used in
///   expressions when invoking macro, since there is no `self` at invocation place.
/// - **$key**: alias for `key` argument. Same reason.
/// - **$capacity**: alias for `capacity` argument. Same reason.
///
/// Arguments per collection type to implement:
/// - **$ty**: collection type name. Generic type parameters must be named `KEY` and `BH`.
/// - **$create**: implementation of [`with_capacity_and_hasher()`][CollectionBuilderTrait::with_capacity_and_hasher()]
///   method
/// - **$insert**: implementation of [`insert`][CollectionTrait::insert()] method
/// - **$get**: implementation of [`get`][CollectionTrait::get()] method
/// - **$len**: implementation of [`len`][CollectionTrait::len()] method
/// - **$clear**: implementation of [`clear`][CollectionTrait::clear()] method
///
/// Usage example for [`std HashSet`][std::collections::HashSet] wrapper [`CollectionStdHashSet`]:
///
///     CollectionStdHashset<KEY,BH> =>
///         create  = Self(std::collections::HashSet::<KEY, BH>::with_capacity_and_hasher(capacity, build_hasher)),
///         insert  = this.0.insert(key),
///         get     = this.0.get(key),
///         len     = this.0.len(),
///         clear   = this.0.clear();
macro_rules! impl_hashset_traits {
    ($self:ident,$key:ident,$capacity:ident,$build_hasher:ident,
        $($ty:ty => create = $create:expr, insert = $insert:expr, get = $get:expr, len = $len:expr, clear = $clear:expr);+) => {
        $(
            #[allow (unused_variables, reason = "Some implementations don't use capacity or / and hasher")]
            impl<BH: BuildHasher+Clone+'static> CollectionBuilderTrait<BH> for $ty {
                fn with_capacity_and_hasher(capacity: usize, build_hasher: BH) -> Self {
                    let $capacity = capacity;
                    let $build_hasher = build_hasher;
                    $create
                }
            }

            #[allow (unused_variables, reason = "Some implementations don't use self")]
            impl<BH: BuildHasher+Clone+'static> CollectionTrait for $ty {
                fn insert(&mut self, key: KeyVal) {
                    let $self = self;
                    let $key = key;
                    $insert;
                }
                fn get<'a: 'b, 'b>(&'a self, key: &'b KeyVal) -> Option<&'b KeyVal> {
                    let $self = self;
                    let $key = key;
                    $get
                }
                fn len(&self) -> usize { let $self = self; $len }
                fn clear(&mut self) { let $self = self; $clear }
            }
        )+
    };
}

impl_hashset_traits! (this, key, capacity, build_hasher,
    CollectionStdHashSet<BH> =>
        create  = Self(std::collections::HashSet::<KeyVal,BH>::with_capacity_and_hasher(capacity, build_hasher)),
        insert  = this.0.insert(key),
        get     = this.0.get(key),
        len     = this.0.len(),
        clear   = this.0.clear();
    CollectionHashbrownHashSet<BH> =>
        create  = Self(hashbrown::HashSet::<KeyVal,BH>::with_capacity_and_hasher(capacity, build_hasher)),
        insert  = this.0.insert(key),
        get     = this.0.get(key),
        len     = this.0.len(),
        clear   = this.0.clear();
    CollectionBTreeSet<BH> =>
        create  = Self(BTreeSet::<KeyVal>::new(), PhantomData),
        insert  = this.0.insert(key),
        get     = this.0.get(key),
        len     = this.0.len(),
        clear   = this.0.clear();
    CollectionLitemap<BH> =>
        create  = Self(LiteMap::<KeyVal, ()>::with_capacity(capacity), PhantomData),
        insert  = this.0.insert(key, ()),
        get     = this.0.get(key).map(|()| key),
        len     = this.0.len(),
        clear   = this.0.clear();
    CollectionVecSet<BH> =>
        create  = Self(vecmap::VecSet::<KeyVal>::with_capacity(capacity), PhantomData),
        insert  = this.0.insert(key),
        get     = this.0.get(key),
        len     = this.0.len(),
        clear   = this.0.clear();
    CollectionVectorMap<BH> =>
        create  = Self(vector_map::VecMap::<KeyVal, ()>::with_capacity(capacity), PhantomData),
        insert  = this.0.insert(key, ()),
        get     = this.0.get(key).map(|()| key),
        len     = this.0.len(),
        clear   = this.0.clear();
    CollectionDummy<BH> =>
        create  = Self(build_hasher),
        insert  = { },
        get     = {
            key.clone().hash_key(&this.0.clone());
            None
        },
        len     = 0,
        clear   = { });


/// Timer powered by either [`Instant`] or [`SystemTime`] to measure durations
///
/// Which timer type gets used is specified by `--timer` argument and corresponding [`TimerSourceEnum`] value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Timer {
    /// Use stdlib [`Instant`] as timing source.
    ///
    /// - guaranteed not to fail, i.e. always counts forwards, barring platform bugs
    /// - not guaranteed to be steady, i.e. some seconds may be longer than others
    /// - potentially higher nanoseconds precision
    Instant(Instant),
    /// Use stdlib [`SystemTime`] as timing source.
    ///
    /// - not guaranteed not to fail, i.e. 2nd query may yield time before 1st one
    /// - guaranteed to be steady, i.e. all seconds are equally long
    /// - potentially lower nanoseconds precision
    SystemTime(SystemTime),
}

impl Timer {
    /// Instantiate a new [`Timer`] instance by given [`TimerSourceEnum`] type
    fn new(timer_type: TimerSourceEnum) -> Self {
        match timer_type {
            | TimerSourceEnum::Instant => Self::Instant(Instant::now()),
            | TimerSourceEnum::SysTime => Self::SystemTime(SystemTime::now()),
        }
    }

    /// Restart timer
    fn restart(&mut self) {
        match self {
            | &mut Self::Instant(ref mut i) => *i = Instant::now(),
            | &mut Self::SystemTime(ref mut s) => *s = SystemTime::now(),
        }
    }

    /// Get elapsed duration since creation or last [`Self::restart()`] call in nanoseconds
    fn get_elapsed_ns(&self) -> u64 {
        match self {
            | Self::Instant(i) => u64::try_from(i.elapsed().as_nanos()).unwrap_or(u64::from(u32::MAX)),
            | Self::SystemTime(s) => u64::try_from(s.elapsed().unwrap_or(Duration::from_micros(1)).as_nanos()).unwrap_or(u64::from(u32::MAX)),
        }
    }
}



/// Whether a benchmark executor is running in warmup or measure mode
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
#[repr(u8)]
enum ExecutorWarmupMode {
    /// This is a warmup run, results are ignored.
    Warmup,
    /// This is actual measurement run
    Measure,
}


/// Describes benchmark operations to execute / which were executed
#[derive(Debug, Clone, PartialEq, Eq)]
struct OperationsDescription {
    num_permutations:      u32,
    permutation_run_ms:    u16,
    permutation_warmup_ms: u16,
    min_duration_ms:       u32,
    threads:               u8,
    max_aberration:        u8,
    set_sizes:             Vec<u32>,
    offsets:               Vec<u32>,
    steps:                 Vec<u32>,
    key_types:             Vec<KeyType>,
    string_key_lengths:    Vec<u16>,
    collections:           Vec<CollectionType>,
    hashers:               Vec<HasherEnum>,
    operations:            Vec<HashOp>,
    hit_rates_perc:        Vec<u8>,
    exec_sort_order:       Vec<SortBy>,
    cpu_name:              String,
    cpu_mhz:               u16,
    cpu_cores:             u8,
    free_ram_mb:           u32,
    binary_build_mode:     BinaryBuildMode,
}

impl Display for OperationsDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Self::iter_key_value_pairs().try_for_each(|(key, val_fn)| {
            let value = val_fn(self);
            if value.is_empty() { writeln!(f, "{key}") } else { writeln!(f, "  {key:-24}: {value}") }
        })
    }
}

impl OperationsDescription {
    /// Create instance of this description
    #[allow(clippy::expect_used, reason = "System error if no CPU info could be found")]
    #[allow(clippy::cast_possible_truncation, reason = "Not expecting more than 100 threads or a petabyte of RAM")]
    fn create(main: &Main) -> Self {
        let args = &main.args;
        let permspec = &args.permutation_spec;
        let cpu_arch = sysinfo::System::cpu_arch();
        let cpu = main.sysinfo.cpus().first().expect("No CPU info found!");
        let cpu_vendor = cpu.vendor_id();
        let cpu_model = cpu.brand();
        let mut sort_order = main.permutations_sort_order.clone();
        sort_order.reverse();
        Self {
            num_permutations:      main.num_permutations,
            exec_sort_order:       sort_order,
            permutation_run_ms:    args.general.run_ms,
            permutation_warmup_ms: args.general.warmup_ms,
            min_duration_ms:       main.calc_duration_ms(main.num_permutations),
            threads:               args.general.threads,
            max_aberration:        args.general.tolerance,
            set_sizes:             permspec.size.clone(),
            offsets:               permspec.offset.clone(),
            steps:                 permspec.step.clone(),
            key_types:             permspec.keytype.clone(),
            string_key_lengths:    permspec.length.clone(),
            collections:           permspec.collection.clone(),
            hashers:               permspec.hasher.clone(),
            operations:            permspec.op.clone(),
            hit_rates_perc:        permspec.hit_rate.clone(),
            cpu_name:              format!("{cpu_arch} {cpu_vendor} {cpu_model}"),
            cpu_mhz:               main.sysinfo.cpus().first().map_or(0, |cpu| sysinfo::Cpu::frequency(cpu) as u16),
            cpu_cores:             main.sysinfo.cpus().len() as u8,
            free_ram_mb:           (main.sysinfo.available_memory() >> 20) as u32,
            binary_build_mode:     main.build_mode,
        }
    }

    /// Obatin static slice of operation description keys and value generation expression
    #[rustfmt::skip]
    #[allow (clippy::type_complexity, reason = "No custom type just because of tuple with fn pointer")]
    fn iter_key_value_pairs() -> impl Iterator<Item=&'static (&'static str, fn(&OperationsDescription)->String)> {
        let keys_vals: &'static [(&'static str, fn(&OperationsDescription) -> String)] = &[
            ("Configured benchmarks summary",   |_od| String::new()),
            ("Permutations",                    |od| od.num_permutations.to_string()),
            ("Execution ordering",              |od| OutputFormat::slice_to_json_string(&od.exec_sort_order)),
            ("Duration per perm ms",            |od| od.permutation_run_ms.to_string()),
            ("Warmup per perm",                 |od| od.permutation_warmup_ms.to_string()),
            ("Aberration tolerance %",          |od| od.max_aberration.to_string()),
            ("Threads (parallelism)",           |od| od.threads.to_string()),
            ("Minimum bench duration ms",       |od| od.min_duration_ms.to_string()),
            ("Selected parameters",             |_od| String::new()),
            ("Set sizes",                       |od| OutputFormat::slice_to_json_string(&od.set_sizes)),
            ("Offsets",                         |od| OutputFormat::slice_to_json_string(&od.offsets)),
            ("Key steps",                       |od| OutputFormat::slice_to_json_string(&od.steps)),
            ("Key types",                       |od| OutputFormat::slice_to_json_string(&od.key_types)),
            ("String key lengths",              |od| OutputFormat::slice_to_json_string(&od.string_key_lengths)),
            ("Collections",                     |od| OutputFormat::slice_to_json_string(&od.collections)),
            ("Hashers",                         |od| OutputFormat::slice_to_json_string(&od.hashers)),
            ("Operations",                      |od| OutputFormat::slice_to_json_string(&od.operations)),
            ("Hit rates %",                     |od| OutputFormat::slice_to_json_string(&od.hit_rates_perc)),
            ("System info",                     |_od| String::new()),
            ("CPU",                             |od| od.cpu_name.clone()),
            ("CPU frequency MHz",               |od| od.cpu_mhz.to_string()),
            ("Number of CPUs (cores)",          |od| od.cpu_cores.to_string()),
            ("Free RAM",                        |od| od.free_ram_mb.to_string()),
            ("Binary build mode",               |od| od.binary_build_mode.to_string()),
        ];
        keys_vals.iter ()
    }
}



/// "Global" state and information of application. Also contains bulk of code.
///
/// Actually a local variable of [`main()`] and passed down to all code that needs this information. It makes sense to
/// make most program functions methods of this type. Because many methods need [`Args`] information, to configure
/// benchmarks, and [`Main::start_time`], which is used by [`Main::printmsg()`].
#[derive(Debug)]
struct Main {
    /// Whether this binary was build in `Debug` or `Release` mode
    build_mode:              BinaryBuildMode,
    /// [`DateTime`] when this application was started. Used by [`printmsg()`][Main::printmsg()].
    start_time:              DateTime<Local>,
    /// System information, number of available CPUs, available RAM etc.
    sysinfo:                 System,
    /// Command line argument values as evaluated by [`Clap`][clap]
    args:                    Args,
    /// Number of permutations that result from given parameters
    num_permutations:        u32,
    /// Sort order by order of permutation arguments
    permutations_sort_order: Vec<SortBy>,
}

impl Main {
    /// Instantiate [`Main`]. [`Main::main()`] used next.
    fn new() -> Self {
        Self {
            #[cfg(debug_assertions)]
            build_mode: BinaryBuildMode::Debug,
            #[cfg(not(debug_assertions))]
            build_mode: BinaryBuildMode::Release,
            start_time: Local::now(),
            sysinfo: System::new(),
            args: Args::parse(),
            num_permutations: 0,
            permutations_sort_order: Vec::default(),
        }
    }

    /// Main program flow.
    ///
    /// 1. obtain system information
    /// 2. evaluate and validate command line arguments
    /// 3. prepare and execute benchmarks
    /// 4. post-process and emit benchmark results
    fn main(&mut self) -> MyResult<()> {
        self.permutations_sort_order = Self::determine_execution_sort_order()?;
        self.sysinfo.refresh_cpu_list(CpuRefreshKind::everything());
        self.sysinfo.refresh_memory();
        self.expand_args()?;
        self.check_args()?;
        self.precalculate()?;
        let mut benchmark_result = self.benchmark_permutations()?;
        if !self.can_emit_results_immediately() {
            self.sort_benchmark_results(benchmark_result.as_mut_slice());
            self.emit_benchmark_results(benchmark_result.as_slice())?;
        }
        Ok(())
    }

    /// Determines order of benchmark permutations to run.
    ///
    /// Execution order is determined by order of command line arguments, if given. For not explicitly specified
    /// arguments, same order as in [`ArgsPermutation`] will be used.
    ///
    /// This implementation first creates a Vector of type `(SortOrder,u8)`, from which it creates the final result, a
    /// sorted `Vec<SortOrder>`.
    #[allow(clippy::cast_possible_truncation, reason = "We don't need to expect to receive more than 100 CLI args ever")]
    fn determine_execution_sort_order() -> MyResult<Vec<SortBy>> {
        let matches = Args::command().get_matches();
        let mut vec_arg_order: Vec<(SortBy, u8)> =
            [SortBy::Collection, SortBy::Hasher, SortBy::KeyType, SortBy::Size, SortBy::Length, SortBy::Offset, SortBy::Step, SortBy::Op, SortBy::HitRate]
                .iter()
                .try_fold(Vec::with_capacity(10), |mut vec, elem| {
                    let arg_name = elem.arg_name().unwrap_or("-");
                    let order = matches
                        .index_of(arg_name)
                        .ok_or_else(|| Error::new(ErrorKind::Internal, &format!("Cannot find argument: {arg_name}")))? as u8;
                    vec.push((*elem, order));
                    Ok::<alloc::vec::Vec<(SortBy, u8)>, Error>(vec)
                })?;
        vec_arg_order.sort_by_key(|elem| elem.1);
        vec_arg_order.reverse();
        let vec = vec_arg_order.iter().fold(Vec::with_capacity(vec_arg_order.len()), |mut vec, elem| {
            vec.push(elem.0);
            vec
        });
        Ok(vec)
    }

    /// If "all" value found in one value list, replace with all possible values of its kind.
    #[rustfmt::skip]
    fn expand_args(&mut self) -> MyResult<()> {
        let perm = &mut self.args.permutation_spec;
        perm.collection = CollectionType::expand(perm.collection.as_slice())?;
        perm.hasher     = HasherEnum    ::expand(perm.hasher.as_slice())?;
        perm.keytype    = KeyType       ::expand(perm.keytype.as_slice())?;
        Ok(())
    }

    /// Check that command line arguments contain valid and nonempty values
    #[allow(clippy::integer_division, reason = "Precision loss irrelevant")]
    #[allow(clippy::cast_possible_truncation, reason = "On 32 bit systems no more than 4 GB RAM can be available anyway")]
    #[rustfmt::skip]
    fn check_args(&mut self) -> MyResult<()> {
        /// Ensure that argument list contains at least one element.
        fn assert_nonempty_arglist<T>(list: &[T], name: &str) -> MyResult<()> {
            (!list.is_empty()).then_some(()).ok_or_else(|| Error::new(ErrorKind::Args, &format!("Empty list for argument: {name}")))
        }

        let args = &self.args;
        let perm = &args.permutation_spec;
        assert_nonempty_arglist(&perm.size,         "set sizes")?;
        assert_nonempty_arglist(&perm.collection,   "collection")?;
        assert_nonempty_arglist(&perm.hasher,       "hasher")?;
        assert_nonempty_arglist(&perm.keytype,      "key type")?;
        assert_nonempty_arglist(&perm.op,           "hashset operation")?;
        assert_nonempty_arglist(&perm.hit_rate,     "hit rate %")?;
        assert_nonempty_arglist(&perm.length,       "String lengths")?;
        assert_nonempty_arglist(&perm.step,         "Add values")?;
        assert_nonempty_arglist(&perm.offset,       "offset")?;
        for size in &perm.size {
            for step in &perm.step {
                for offset in    &perm.offset {
                    if u64::from(*size) * u64::from(*step) + u64::from(*offset) >= u64::from(u32::MAX) {
                        return Err(Error::new(ErrorKind::Args, "Product of size * add + offset exceeds u32 range in at least one combination."));
                    }
                }
            }
        }
        Ok(())
    }

    /// Precalculate some numbers for upcoming benchmarks
    #[allow(clippy::cast_possible_truncation, reason = "No more than 4 billion permutations expected + supported")]
    #[allow(clippy::integer_division, reason = "Just need to calculate 10% of available memory")]
    fn precalculate(&mut self) -> MyResult<()> {
        self.num_permutations = self.args.permutation_spec.into_iter().count() as u32;
        let num_permutations: u32 = self.num_permutations;
        let duration_ms = self.calc_duration_ms(num_permutations);
        let ram_demand = self.calc_ram_demand_kb();
        self.print_operations_summary();
        self.check_target_files_exist_without_overwrite()?;
        if (num_permutations > 1000 ||
            duration_ms > 10 * 60 * 1000 ||
            ram_demand as usize * 1024 > self.sysinfo.free_memory() as usize / 10 ||
            u64::from(*self.args.permutation_spec.length.iter().max().unwrap_or(&0)) *
                u64::from(*self.args.permutation_spec.size.iter().max().unwrap_or(&0)) >
                2_000_000 ||
            self.args.general.threads as usize > self.sysinfo.cpus().len()) &&
            !self.args.general.yes
        {
            Err(Error::new(
                ErrorKind::Args,
                "Very many or long permutations or more threads than available CPUs selected or long runtime expected. Use -y to execute anyway.",
            ))?;
        }
        Ok(())
    }

    /// Validate correct implementation of various `expand()` methods
    fn assert_correct_len<T>(expected_len: usize, collection_name: &str, collection: Vec<T>) -> Result<Vec<T>, Error> {
        (collection.len() == expected_len)
            .then_some(collection)
            .ok_or(Error::new(ErrorKind::Internal, &format!("Software error: {collection_name}::expand() function creates wrong result!")))
    }

    /// Deduplicate entries from arguments parameters.
    ///
    /// Simple yet slightly more expensive approach by copying list of values. Yet since we expect less than 10 items on
    /// most lists, this is irrelevant.
    fn dedup<T: Copy+PartialEq>(collection: &[T]) -> Vec<T> {
        collection.iter().fold(Vec::with_capacity(collection.len()), |mut acc, elem| {
            if !acc.contains(elem) {
                acc.push(*elem);
            }
            acc
        })
    }

    /// Calculate expected total duration of all benchmarks, according to given number of CPUs and specified threads
    #[allow(clippy::integer_division, reason = "Precision loss irrelevant")]
    #[allow(clippy::cast_possible_truncation, reason = "We don't expect more than 4 billion CPUs")]
    fn calc_duration_ms(&self, num_permutations: u32) -> u32 {
        let num_cpus = self.sysinfo.cpus().len() as u32;
        let effective_threads = min(u32::from(self.args.general.threads), num_cpus);
        let duration_ms_per_permutation = num_permutations * (u32::from(self.args.general.warmup_ms) + u32::from(self.args.general.run_ms) + 1);
        duration_ms_per_permutation / effective_threads * (100 + effective_threads) / 100
    }

    /// Calculate expected amount of preallocated result data. Does not regard collection contents demand.
    #[allow(clippy::integer_division, reason = "Loss precision irrelevant")]
    #[allow(clippy::cast_possible_truncation, reason = "On 32 bit systems no more than 4 GB RAM can be available anyway")]
    const fn calc_ram_demand_kb(&self) -> u32 {
        (self.num_permutations * (size_of::<BenchmarkResult>() + size_of::<PermutationSpec>()) as u32 * 21 / 20).div_ceil(1024)
    }

    /// Main entry function to begin benchmarks for all permutations as per parameter selection.
    ///
    /// This method checks whether single-threaded or multi-threaded execution is requested. It calls respective
    /// executor methods [`Self::benchmark_permutations_single_threaded()`] or
    /// [`Self::benchmark_permutations_multi_threaded()`]
    fn benchmark_permutations(&self) -> MyResult<Vec<BenchmarkResult>> {
        let num_threads = if self.args.general.threads > 0 { self.args.general.threads as usize } else { self.sysinfo.cpus().len() };
        match num_threads {
            | 0 => Err(Error::new(ErrorKind::Internal, "Software error: zero threads should not be specifyable!")),
            | 1 => self.benchmark_permutations_single_threaded(),
            | _ => self.benchmark_permutations_multi_threaded(),
        }
    }

    /// Single-threaded executor for all benchmark permutations.
    fn benchmark_permutations_single_threaded(&self) -> MyResult<Vec<BenchmarkResult>> {
        self.args
            .permutation_spec
            .into_iter()
            .try_fold(Vec::with_capacity(self.num_permutations as usize), |mut results_vec, ref permutation| {
                results_vec.push(self.execute_benchmark(permutation)?);
                Ok(results_vec)
            })
    }

    /// Multi-threaded executor for all benchmark permutations.
    fn benchmark_permutations_multi_threaded(&self) -> MyResult<Vec<BenchmarkResult>> {
        let results_vec: Mutex<Vec<BenchmarkResult>> = Mutex::new(Vec::with_capacity(self.num_permutations as usize));
        let permutations_iter = Mutex::new(self.args.permutation_spec.into_iter());
        let thread_idx = AtomicU8::new(1);
        let got_error = Mutex::new(None);
        // We're using local thread scope to parallelize. Rayon was tested and dismissed, because too much boilerplate to
        // implement own parallel iterator source.
        std::thread::scope(|thread_scope| {
            for _ in 0..self.args.general.threads {
                thread_scope.spawn(|| {
                    // Can't use Iterator::try_fold() here because the iterator is wrapped by MutexLock
                    self.printmsg(Verbosity::Ops, &format!("Spawned thread {}", thread_idx.fetch_add(1, core::sync::atomic::Ordering::Relaxed)));
                    let mut maybe_permutation = permutations_iter.lock().next();
                    while got_error.lock().is_none() &&
                        let Some(permutation) = maybe_permutation
                    {
                        maybe_permutation = permutations_iter.lock().next();
                        match self.execute_benchmark(&permutation) {
                            | Ok(bmr) => results_vec.lock().push(bmr),
                            | Err(err) => *got_error.lock() = Some(err),
                        }
                    }
                });
            }
        });
        match core::mem::take(&mut *got_error.lock()) {
            | None => Ok(core::mem::take(&mut *results_vec.lock())),
            | Some(error) => Err(error),
        }
    }

    /// Execute one benchmark permutation; single- or multi-threaded.
    ///
    /// Recurses into following levels to instantiate hasher, collection and key types and execute actual benchmark for
    /// current permutation. Then it just returns benchmark result. This and all invoked methods may be executed in
    /// either single- or multi-threaded context.
    fn execute_benchmark(&self, permutation: &PermutationSpec) -> MyResult<BenchmarkResult> {
        let mut timer = Timer::new(self.args.general.timer);
        let mut collection = Self::create_collection(permutation)?;
        self.benchmark_permutation(permutation, &mut collection, &mut timer)
    }

    /// Create collection of specified type and with specified hasher.
    fn create_collection(permutation: &PermutationSpec) -> MyResult<Collection> {
        /// Extract hasher and descend.
        ///
        /// This method creates generic type variations from a runtime parameter. Method can't return with created results
        /// directly, since they represent different generic types each, and there is no object safety 😢 Instead it invokes
        /// subsequent generic processing method [`Main::resolve_key_type()`] directly.
        #[rustfmt::skip]
        fn resolve_hasher(permutation: &PermutationSpec) -> MyResult<Collection> {
            use HasherEnum as HE;
            use BuildHasherDefault as BHD;
            match permutation.hasher {
                | HE::Adler32       => resolve_collection(permutation, BHD::<Adler32>           ::new()),
                | HE::AHasher       => resolve_collection(permutation, BHD::<AHasher>           ::new()),
                | HE::Bricolage     => resolve_collection(permutation, BHD::<Bricolage>         ::new()),
                | HE::City32        => resolve_collection(permutation, BHD::<CityHasher32>      ::new()),
                | HE::City64        => resolve_collection(permutation, BHD::<CityHasher64>      ::new()),
                | HE::City128       => resolve_collection(permutation, BHD::<CityHasher128>     ::new()),
                | HE::DJB2          => resolve_collection(permutation, BHD::<DJB2Hasher>        ::new()),
                | HE::Farm          => resolve_collection(permutation, BHD::<FarmHasher>        ::new()),
                | HE::FNV1a32       => resolve_collection(permutation, BHD::<FNV1aHasher32>     ::new()),
                | HE::FNV1a64       => resolve_collection(permutation, BHD::<FNV1aHasher64>     ::new()),
                | HE::FoldhashFast  => resolve_collection(permutation, foldhash::fast::FixedState::with_seed(0)),
                | HE::FoldhashQual  => resolve_collection(permutation, foldhash::quality::FixedState::with_seed(0)),
                | HE::Fx32          => resolve_collection(permutation, BHD::<FxHasher32>        ::new()),
                | HE::Fx64          => resolve_collection(permutation, BHD::<FxHasher64>        ::new()),
                | HE::HashHasher    => resolve_collection(permutation, BHD::<HashHasher>        ::new()),
                | HE::Highway       => resolve_collection(permutation, BHD::<HighwayHasher>     ::new()),
                | HE::IntHasher     => resolve_collection(permutation, BHD::<IntHasher<u64>>    ::new()),
                | HE::Lookup3       => resolve_collection(permutation, BHD::<Lookup3Hasher>     ::new()),
                | HE::Metro64       => resolve_collection(permutation, BHD::<MetroHash64>       ::new()),
                | HE::Metro128      => resolve_collection(permutation, BHD::<MetroHash128>      ::new()),
                | HE::Murmur32      => resolve_collection(permutation, BHD::<Murmur3Hasher>     ::new()),
                | HE::Murmur128     => resolve_collection(permutation, BHD::<Murmur3Hasher128>  ::new()),
                | HE::Murmur128x64  => resolve_collection(permutation, BHD::<Murmur3Hasher128x64>::new()),
                | HE::OAAT          => resolve_collection(permutation, BHD::<OAATHasher>        ::new()),
                | HE::Rapid         => resolve_collection(permutation, BHD::<RapidHasher>       ::new()),
                | HE::SDBM          => resolve_collection(permutation, BHD::<SDBMHasher>        ::new()),
                | HE::Sea           => resolve_collection(permutation, BHD::<SeaHasher>         ::new()),
                | HE::Sip           => resolve_collection(permutation, BHD::<SipHasher>         ::new()),
                | HE::Sip13         => resolve_collection(permutation, BHD::<SipHasher13>       ::new()),
                | HE::Spooky        => resolve_collection(permutation, BHD::<SpookyHasher>      ::new()),
                | HE::Wy1           => resolve_collection(permutation, BHD::<wyhash::WyHash>    ::new()),
                | HE::Wy3           => resolve_collection(permutation, BHD::<wyhash::final3::WyHash>::new()),
                | HE::XX3_64        => resolve_collection(permutation, BHD::<XxHash3_64>        ::new()),
                | HE::XX32          => resolve_collection(permutation, BHD::<XxHash32>          ::new()),
                | HE::XX64          => resolve_collection(permutation, BHD::<XxHash64>          ::new()),
                | HE::Zwo           => resolve_collection(permutation, BHD::<ZwoHasher>         ::new()),
                | HE::All           => Err (Error::new (ErrorKind::Internal, "Software error: this code should never be reached! (Hasher creation)"))?,
            }
        }

        #[rustfmt::skip]
        fn resolve_collection<BH>(permutation: &PermutationSpec, build_hasher: BH) -> MyResult<Collection>
        where
            BH: BuildHasher+Clone+Debug+'static, {
            use CollectionType as CT;
            use RandomState as RS;
            match permutation.collection {
                | CT::None      => create::<CollectionDummy             <BH>, BH>(permutation, build_hasher),
                | CT::Std       => create::<CollectionStdHashSet        <BH>, BH>(permutation, build_hasher),
                | CT::Hashbrown => create::<CollectionHashbrownHashSet  <BH>, BH>(permutation, build_hasher),
                | CT::BtreeSet  => create::<CollectionBTreeSet          <RS>, RS>(permutation, RS::new()),
                | CT::Litemap   => create::<CollectionLitemap           <RS>, RS>(permutation, RS::new()),
                | CT::VecMap    => create::<CollectionVecSet            <RS>, RS>(permutation, RS::new()),
                | CT::VectorMap => create::<CollectionVectorMap         <RS>, RS>(permutation, RS::new()),
                | CT::All       => Err(Error::new(ErrorKind::Internal, "Software error: this code should never be reached! (Collection<> creation)"))?,
            }
        }

        /// Create collection instance as trait object.
        ///
        /// Done so because saves nearly 50% of compile time compared to keeping it generic down the way. This may
        /// slightly increase overhead / latency in benchmarks due to inline optimizations becoming impossible.
        /// Yet to supposedly insignificant extent.
        ///
        /// Also the same collection instance has to be reused. Information which concrete collection type to
        /// instantiate is lost beyond here.
        #[allow(trivial_casts, reason = "Needed by ValueA::new_stable()")]
        fn create<COLL, BH>(permutation: &PermutationSpec, build_hasher: BH) -> MyResult<Collection>
        where
            COLL: CollectionBuilderTrait<BH>+Debug+'static,
            BH: BuildHasher+Clone+Debug, {
            alloc_stack!(dyn CollectionTrait, [usize; 8], COLL::with_capacity_and_hasher(permutation.size as usize, build_hasher))
        }

        resolve_hasher(permutation)
    }

    /// Benchmark a particular permutation. Collection with respective hasher and key type has been constructed before,
    /// yet content may be rebuilt.
    ///
    /// This method may be executed either single- or multi-threaded.
    #[allow(clippy::unnecessary_wraps, reason = "Upper levels are fallible hence require MyResult although this method is not expected to be fallible")]
    fn benchmark_permutation(&self, permutation: &PermutationSpec, collection: &mut Collection, timer: &mut Timer) -> MyResult<BenchmarkResult> {
        self.printmsg(Verbosity::Ops, &format!("{permutation:?}"));
        let string_keys = self.prepare_random_string_keys_conditionally(permutation, timer);
        debug_assert_eq!(permutation.key_type.is_int_key(), string_keys.is_empty());
        // Very verbose and rather unneeded debug output
        if self.args.output.verbosity >= Verbosity::Values {
            let count = string_keys.len();
            self.printmsg(Verbosity::Values, &format!("Created {count} string keys of length {}", permutation.string_length));
            string_keys.iter().enumerate().for_each(|(idx, key)| {
                self.printmsg(Verbosity::Values, &format!("Key {idx} of {count}: {key}"));
            });
        }
        if self.args.general.warmup_ms > 0 {
            self.benchmark_for_duration(permutation, collection, string_keys.as_slice(), u32::from(self.args.general.warmup_ms), ExecutorWarmupMode::Warmup)?;
        }
        self.benchmark_for_duration(permutation, collection, string_keys.as_slice(), u32::from(self.args.general.run_ms), ExecutorWarmupMode::Measure)
    }

    /// Run one permutation for up to the given maximum duration.
    ///
    /// Ongoing runs are not interrupted. So total running time is always somewhat larger than given
    /// [`ArgsOps::run_ms`].
    ///
    /// This method may be executed either single- or multi-threaded.
    #[allow(clippy::cast_precision_loss, reason = "Just values between 0..=100 needed")]
    #[allow(clippy::unnecessary_wraps, reason = "Prefer to keep fallibility and its downstream handling; may be needed in some future context")]
    fn benchmark_for_duration(
        &self,
        permutation: &PermutationSpec,
        collection: &mut Collection,
        string_keys: &[String],
        run_ms: u32,
        warmup_mode: ExecutorWarmupMode,
    ) -> MyResult<BenchmarkResult> {
        let max_nanos = u64::from(run_ms) * 1_000_000;
        let mut durations_ns = Vec::<u64>::with_capacity(1 << 16);
        let curr_index = permutation.index;
        let max_index = self.num_permutations;
        let percent_done = curr_index as f32 * 100.0 / max_index as f32;
        let timer_type = self.args.general.timer;
        permutation.op.prepare(permutation, collection, string_keys);
        let timer_permutation = Timer::new(timer_type);
        while timer_permutation.get_elapsed_ns() < max_nanos {
            collection.clear();
            let timer_iteration = Timer::new(timer_type);
            permutation.op.execute(permutation, collection, string_keys);
            durations_ns.push(timer_iteration.get_elapsed_ns());
        }
        match warmup_mode {
            // Ignore measurement result
            | ExecutorWarmupMode::Warmup => Ok(BenchmarkResult::default()),
            // Postprocess and use result
            | ExecutorWarmupMode::Measure => {
                let result = BenchmarkResult::create_from_raw_data(self, permutation, &mut durations_ns);
                let format_result = || if self.args.output.human_readable { format!("{result:#?}") } else { format!("{result:?}") };
                let progress_message = if self.args.output.verbosity >= Verbosity::Ops {
                    &format!("Permutation {curr_index} of {max_index} ({percent_done:5.1}%) {}", format_result())
                } else {
                    &format!(
                        "Permutation {curr_index} of {max_index} ({percent_done:5.1}%); filtering out {} of {} results took {} ms",
                        result.dropped,
                        result.dropped + result.iterations,
                        result.filter_duration_ms
                    )
                };
                self.printmsg(Verbosity::Progress, progress_message);
                Ok(result)
            },
        }
    }

    /// Prepare given number of random, printable strings if string keytype is used.
    fn prepare_random_string_keys_conditionally(&self, permutation: &PermutationSpec, timer: &mut Timer) -> Vec<String> {
        match permutation.key_type {
            | KeyType::String => self.prepare_random_string_keys(permutation.string_length, permutation.size, timer),
            | _ => vec![],
        }
    }

    /// Prepare given number of random, printable strings.
    ///
    /// Used as keys to lookup. Length of string may be specified. Should not be less than 8 to
    /// avoid collisions. Uses weak but fast random algorithm and produces visible 7-bit Ascii
    /// characters only.
    #[allow(clippy::unwrap_used, reason = "Conversion to char guaranteed to succeed. Only using byte values in range 0x21..0x7f.")]
    #[allow(clippy::integer_division, reason = "Loss of precision is the idea here")]
    fn prepare_random_string_keys(&self, string_length: u16, count: u32, timer: &mut Timer) -> Vec<String> {
        timer.restart();
        let result = (0..count).fold(Vec::with_capacity(count as usize), |mut vec, _elem| {
            vec.push((0..string_length).fold(String::with_capacity(string_length as usize), |mut string, _| {
                string.push(char::from_u32(u32::from(<u8 as Random>::random(&mut DefaultRandomSource) % 0x5e) + 0x21).unwrap());
                string
            }));
            vec
        });
        self.printmsg(Verbosity::Ops, &format!("Preparing {count} random strings of length {string_length} took {} ms", timer.get_elapsed_ns() / 1_000_000));
        result
    }

    /// Whether benchmark results can be written to targets immediately or only after completion of all benchmarks.
    const fn can_emit_results_immediately(&self) -> bool { self.args.output.sort.is_empty() && !self.args.output.reverse_sort }

    /// Sort list of [`BenchmarkResult`]s, by criteria specified by `--sort` argument.
    fn sort_benchmark_results(&self, benchmark_results: &mut [BenchmarkResult]) {
        let sort_by = self.args.output.sort.as_slice();
        benchmark_results.sort_by(|e1: &BenchmarkResult, e2: &BenchmarkResult| {
            sort_by
                .iter()
                .try_fold(Ordering::Equal, |_, sort_key| {
                    #[rustfmt::skip]
                    let ordering = match *sort_key {
                        | SortBy::None      => Ordering::Equal,
                        | SortBy::Iterations=> e1.iterations                .cmp(&e2.iterations),
                        | SortBy::Time      => e1.nanos_avg                 .cmp(&e2.nanos_avg),
                        | SortBy::TimeMin   => e1.nanos_min                 .cmp(&e2.nanos_min),
                        | SortBy::TimeMax   => e1.nanos_max                 .cmp(&e2.nanos_max),
                        | SortBy::Index     => e1.permutation.index         .cmp(&e2.permutation.index),
                        | SortBy::Collection=> e1.permutation.collection    .cmp(&e2.permutation.collection),
                        | SortBy::Hasher    => e1.permutation.hasher        .cmp(&e2.permutation.hasher),
                        | SortBy::KeyType   => e1.permutation.key_type      .cmp(&e2.permutation.key_type),
                        | SortBy::Size      => e1.permutation.size          .cmp(&e2.permutation.size),
                        | SortBy::Length    => e1.permutation.string_length .cmp(&e2.permutation.string_length),
                        | SortBy::Offset    => e1.permutation.offset        .cmp(&e2.permutation.offset),
                        | SortBy::Step      => e1.permutation.step          .cmp(&e2.permutation.step),
                        | SortBy::Op        => e1.permutation.op            .cmp(&e2.permutation.op),
                        | SortBy::HitRate   => e1.permutation.hit_rate_perc .cmp(&e2.permutation.hit_rate_perc),
                    };
                    // self.printmsg(
                    //     Verbosity::Ops,
                    //     &format!("Sorting by key: {sort_key}, result: {ordering:?}, key 1: {}, key 2: {}, time1: {}, time2: {}",
                    // e1.iterations, e2.iterations,e1.nanos_avg, e2.nanos_avg), );
                    #[rustfmt::skip]
                    let ordering_result = match ordering {
                        | Ordering::Equal   => Ok(Ordering::Equal),
                        | Ordering::Greater => Err(Ordering::Greater),
                        | Ordering::Less    => Err(Ordering::Less),
                    };
                    ordering_result
                })
                .unwrap_or_else(|ordering| ordering)
        });
        if self.args.output.reverse_sort {
            benchmark_results.reverse();
        }
    }

    fn emit_benchmark_results(&self, benchmark_results: &[BenchmarkResult]) -> MyResult<()> {
        let output_args = &self.args.output;
        output_args.format.iter().try_for_each(|output_format| {
            let mut writer = self.create_target_writer(*output_format)?;
            output_format.write_header(&mut *writer, &OperationsDescription::create(self), output_args.csv_header, output_args.output_mode)?;
            benchmark_results.iter().enumerate().try_for_each(|(idx, benchmark_result)| {
                output_format.write_benchmark_result(&mut *writer, benchmark_result, output_args.output_mode, idx == 0)
            })?;
            output_format.write_footer(&mut *writer, output_args.output_mode)
        })
    }

    /// Check whether result target files exist and neither `--overwrite` nor `-y` have been specified.
    fn check_target_files_exist_without_overwrite(&self) -> MyResult<()> {
        let has_multiple_formats = self.args.output.format.len() > 1;
        if self.args.output.output_file.is_some() && !self.args.output.overwrite && !self.args.general.yes {
            self.args
                .output
                .format
                .iter()
                .try_for_each(|output_format| self.check_target_file_exists_without_overwrite(*output_format, has_multiple_formats))
        } else {
            Ok(())
        }
    }

    /// Check whether result target file exists and neither `--overwrite` nor `-y` have been specified.
    fn check_target_file_exists_without_overwrite(&self, output_format: OutputFormat, has_multiple_formats: bool) -> MyResult<()> {
        if let Some(filename) = self.args.output.output_file.as_ref() &&
            !self.args.output.overwrite &&
            !self.args.general.yes
        {
            let filespec = &Self::create_result_filespec(filename, output_format, has_multiple_formats);
            (!std::fs::exists(filespec)?).then_some(()).ok_or_else(|| {
                Error::new(
                    ErrorKind::Args,
                    &format!("Target file already exists: {}. Specify --overwrite or -y to overwrite.", filespec.to_str().unwrap_or("!!!")),
                )
            })
        } else {
            Ok(())
        }
    }

    /// Create filespec for result file from given arguments.
    ///
    /// If multiple output formats are selected and target file is specified, then format-specific suffix (e.g. `.csv`)
    /// is appended to filename.
    fn create_result_filespec(given_filename: &Path, output_format: OutputFormat, has_multiple_formats: bool) -> PathBuf {
        static DEFAULT_FILENAME: &str = "result";
        if has_multiple_formats {
            PathBuf::from(format!("{}{}", given_filename.to_str().unwrap_or(DEFAULT_FILENAME), output_format.filename_suffix()))
        } else {
            given_filename.to_path_buf()
        }
    }

    /// Create target writer (stdout or file) to write benchmark results into.
    ///
    /// - if no target file is specified by `-o`, writer to `stdout` is returned.
    /// - if multiple output formats are selected and target file is specified, then format-specific suffix (e.g.
    ///   `.csv`) is appended to filename.
    /// - if target file exists, it won't be overwritten unless either `--overwrite` or `-y` is specified.
    #[allow(trivial_casts, reason = "Needed by Value allocation")]
    fn create_target_writer(&self, output_format: OutputFormat) -> MyResult<Writer> {
        let has_multiple_formats = self.args.output.format.len() > 1;
        self.check_target_file_exists_without_overwrite(output_format, has_multiple_formats)?;
        let optional_filespec = &self.args.output.output_file;
        let overwrite = self.args.output.overwrite || self.args.general.yes;
        let filespec = optional_filespec
            .as_ref()
            .map_or(PathBuf::from(""), |fs| Self::create_result_filespec(fs, output_format, has_multiple_formats));
        match (optional_filespec, overwrite) {
            | (None, ..) => alloc_stack!(dyn Write, [usize; 4], stdout()),
            | (Some(_), true) => alloc_stack!(dyn Write, [usize; 4], File::create(filespec)?),
            | (Some(_), false) => alloc_stack!(dyn Write, [usize; 4], File::create_new(filespec)?),
        }
    }

    /// Print summary of specified operation to stdout (if verbosity grants it)
    #[allow(clippy::integer_division, reason = "Precision loss irrelevant for rough output")]
    fn print_operations_summary(&self) -> bool {
        self.printmsg(Verbosity::Info, &OperationsDescription::create(self).to_string())
        // TODO: remove this old implementation
        // fn slice_to_string<T: IntoIterator>(slice: T) -> String
        // where <T as IntoIterator>::Item: Display {
        //     slice.into_iter().map(|slice| slice.to_string()).collect::<Vec<String>>().join(", ")
        // }
        // let args = &self.args;
        // let perm = &args.permutation_spec;
        // let num_permutations: u32 = self.num_permutations;
        // let duration_ms = self.calc_duration_ms(num_permutations);
        // let duration_perm_ms = self.args.general.run_ms;
        // let duration_warmup_ms = self.args.general.warmup_ms;
        // let minutes = duration_ms / 60000;
        // let seconds = (duration_ms % 60000) / 1000;
        // let ram_demand_kb = self.calc_ram_demand_kb();
        // let num_threads: u8 = self.args.general.threads;
        // let cpu_arch = sysinfo::System::cpu_arch();
        // let cpu = self.sysinfo.cpus().first().ok_or_else(|| Error::new(ErrorKind::Internal, "No CPU info found!"))?;
        // let cpu_vendor = cpu.vendor_id();
        // let cpu_model = cpu.brand();
        // let cpu_frequency = self.sysinfo.cpus().first().map_or(0, sysinfo::Cpu::frequency);
        // let num_cpus = self.sysinfo.cpus().len();
        // let free_ram_mb = self.sysinfo.available_memory() >> 20;
        // let set_sizes = slice_to_string(&perm.size);
        // let offsets = slice_to_string(&perm.offset);
        // let steps = slice_to_string(&perm.step);
        // let key_types = slice_to_string(&perm.keytype);
        // let string_key_lengths = slice_to_string(&perm.length);
        // let collections = slice_to_string(&perm.collection);
        // let hashers = slice_to_string(&perm.hasher);
        // let ops = slice_to_string(&perm.op);
        // let percent = slice_to_string(&perm.hit_rate);
        // let build_mode = self.build_mode;
        // let mut exec_sort_order = self.permutations_sort_order.clone();
        // exec_sort_order.reverse();
        // for text_line in [
        //     "Configured benchmarks summary:",
        //     &format!("  Permutations:           {num_permutations}"),
        //     &format!("  Execution ordering:     {exec_sort_order:?}"),
        //     &format!("  Duration per perm:      {duration_perm_ms} ms"),
        //     &format!("  Warmup per perm:        {duration_warmup_ms} ms"),
        //     &format!("  Threads (parallelism):  {num_threads}"),
        //     &format!("  Minimum bench duration: {minutes} minutes, {seconds} seconds"),
        //     &format!("  Approx. RAM demand:     {ram_demand_kb} KB"),
        //     "Selected parameters:",
        //     &format!("  Set sizes:              {set_sizes}"),
        //     &format!("  Offsets:                {offsets}"),
        //     &format!("  Key steps:              {steps}"),
        //     &format!("  Key types:              {key_types}"),
        //     &format!("  String key lengths:     {string_key_lengths}"),
        //     &format!("  Collections:            {collections}"),
        //     &format!("  Hashers:                {hashers}"),
        //     &format!("  Operations:             {ops}"),
        //     &format!("  Hit rates %:            {percent}"),
        //     "System info:",
        //     &format!("  CPU:                    {cpu_arch} {cpu_vendor} {cpu_model}"),
        //     &format!("  CPU frequency:          {cpu_frequency} MHz"),
        //     &format!("  Number of CPUs (cores): {num_cpus}"),
        //     &format!("  Free RAM:               {free_ram_mb} MB"),
        //     &format!("Binary was built in {build_mode} mode"),
        // ] {
        //     self.printmsg(Verbosity::Info, text_line);
        // }
    }

    /// Conditionally print text message to stdout, with datetime and runtime in ms prepended
    #[allow(clippy::print_stdout, reason = "This function is meant to print to stdout")]
    fn printmsg(&self, verbosity_level: Verbosity, text: &str) -> bool {
        if self.args.output.verbosity >= verbosity_level {
            let now = Local::now();
            let now_fmt = now.format("%Y-%m-%d %H:%M:%S");
            let secs = (now - self.start_time).as_seconds_f32();
            println!("{now_fmt} # {secs:.3}; {verbosity_level}:  {text}");
            return true;
        }
        false
    }
}



/// Application entry point. Calls [`Main::main()`]
///
/// This function just calls [`Main::new()`], executes [`Main::main()`] and prints error message, if any.
fn main() -> ExitCode {
    let mut main = Main::new();
    match main.main() {
        | Ok(()) => ExitCode::from(ErrorKind::Ok),
        | Err(err) => {
            main.args.output.verbosity = Verbosity::Err;
            main.printmsg(Verbosity::Err, &err.text);
            err.kind.into()
        },
    }
}
