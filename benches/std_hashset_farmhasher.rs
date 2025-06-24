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

use farmhash::FarmHasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(std_hashset);

create_benchmark! (std_hashset,
    std_hashset, HashSet<u32,BuildHasherDefault<FarmHasher>>,       u32,        farmhasher, FarmHasher,
    std_hashset, HashSet<usize,BuildHasherDefault<FarmHasher>>,     usize,      farmhasher, FarmHasher,
    std_hashset, HashSet<u128,BuildHasherDefault<FarmHasher>>,      u128,       farmhasher, FarmHasher,
    std_hashset, HashSet<String,BuildHasherDefault<FarmHasher>>,    String,     farmhasher, FarmHasher,
    std_hashset, HashSet<String8,BuildHasherDefault<FarmHasher>>,   String8,    farmhasher, FarmHasher,
    std_hashset, HashSet<String16,BuildHasherDefault<FarmHasher>>,  String16,   farmhasher, FarmHasher,
    std_hashset, HashSet<String32,BuildHasherDefault<FarmHasher>>,  String32,   farmhasher, FarmHasher,
    std_hashset, HashSet<StringSlow,BuildHasherDefault<FarmHasher>>,StringSlow, farmhasher, FarmHasher
);
