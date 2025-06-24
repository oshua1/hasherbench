#![allow(
    unused_crate_dependencies,
    reason = "Every benchmark module tests single collection; others remain unused"
)]
#![allow(
    non_snake_case,
    reason = "Auto-creating function names with embedded type names like 'String'"
)]
#![allow(
    single_use_lifetimes,
    reason = "impl HashSetTrait for vector_map_VecSet::get() fails without (otherwise unneeded) lifetime annotation"
)]
#![allow(
    clippy::unreadable_literal,
    reason = "Numbers are used for function names; need to avoid ambiguities with following percentage parameter"
)]

mod common;

use core::hash::BuildHasherDefault;

use hashbrown::HashSet;
use metrohash::MetroHash64;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<MetroHash64>>,        u32,        metrohash64,    MetroHash64,
    hashbrown, HashSet<usize,BuildHasherDefault<MetroHash64>>,      usize,      metrohash64,    MetroHash64,
    hashbrown, HashSet<u128,BuildHasherDefault<MetroHash64>>,       u128,       metrohash64,    MetroHash64,
    hashbrown, HashSet<String,BuildHasherDefault<MetroHash64>>,     String,     metrohash64,    MetroHash64,
    hashbrown, HashSet<String8,BuildHasherDefault<MetroHash64>>,    String8,    metrohash64,    MetroHash64,
    hashbrown, HashSet<String16,BuildHasherDefault<MetroHash64>>,   String16,   metrohash64,    MetroHash64,
    hashbrown, HashSet<String32,BuildHasherDefault<MetroHash64>>,   String32,   metrohash64,    MetroHash64,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<MetroHash64>>, StringSlow, metrohash64,    MetroHash64
);
