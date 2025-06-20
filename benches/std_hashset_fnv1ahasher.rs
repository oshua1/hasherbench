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
use std::collections::HashSet;

use hashers::fnv::FNV1aHasher64;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(std_hashset);

create_benchmark! (std_hashset,
    std_hashset, HashSet<u32,BuildHasherDefault<FNV1aHasher64>>,        u32,        fnv1a64,    FNV1aHasher64,
    std_hashset, HashSet<usize,BuildHasherDefault<FNV1aHasher64>>,      usize,      fnv1a64,    FNV1aHasher64,
    std_hashset, HashSet<u128,BuildHasherDefault<FNV1aHasher64>>,       u128,       fnv1a64,    FNV1aHasher64,
    std_hashset, HashSet<String,BuildHasherDefault<FNV1aHasher64>>,     String,     fnv1a64,    FNV1aHasher64,
    std_hashset, HashSet<String8,BuildHasherDefault<FNV1aHasher64>>,    String8,    fnv1a64,    FNV1aHasher64,
    std_hashset, HashSet<String16,BuildHasherDefault<FNV1aHasher64>>,   String16,   fnv1a64,    FNV1aHasher64,
    std_hashset, HashSet<String32,BuildHasherDefault<FNV1aHasher64>>,   String32,   fnv1a64,    FNV1aHasher64,
    std_hashset, HashSet<StringSlow,BuildHasherDefault<FNV1aHasher64>>, StringSlow, fnv1a64,    FNV1aHasher64
);
