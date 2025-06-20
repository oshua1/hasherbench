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

use hashers::oz::DJB2Hasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(std_hashset);

create_benchmark! (std_hashset,
    std_hashset, HashSet<u32,BuildHasherDefault<DJB2Hasher>>,       u32,        djb2hasher, DJB2Hasher,
    std_hashset, HashSet<usize,BuildHasherDefault<DJB2Hasher>>,     usize,      djb2hasher, DJB2Hasher,
    std_hashset, HashSet<u128,BuildHasherDefault<DJB2Hasher>>,      u128,       djb2hasher, DJB2Hasher,
    std_hashset, HashSet<String,BuildHasherDefault<DJB2Hasher>>,    String,     djb2hasher, DJB2Hasher,
    std_hashset, HashSet<String8,BuildHasherDefault<DJB2Hasher>>,   String8,    djb2hasher, DJB2Hasher,
    std_hashset, HashSet<String16,BuildHasherDefault<DJB2Hasher>>,  String16,   djb2hasher, DJB2Hasher,
    std_hashset, HashSet<String32,BuildHasherDefault<DJB2Hasher>>,  String32,   djb2hasher, DJB2Hasher,
    std_hashset, HashSet<StringSlow,BuildHasherDefault<DJB2Hasher>>,StringSlow, djb2hasher, DJB2Hasher
);
