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

use twox_hash::XxHash64;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(std_hashset);

create_benchmark! (std_hashset,
    std_hashset, HashSet<u32,BuildHasherDefault<XxHash64>>,         u32,        xxhash64,   XxHash64,
    std_hashset, HashSet<usize,BuildHasherDefault<XxHash64>>,       usize,      xxhash64,   XxHash64,
    std_hashset, HashSet<u128,BuildHasherDefault<XxHash64>>,        u128,       xxhash64,   XxHash64,
    std_hashset, HashSet<String,BuildHasherDefault<XxHash64>>,      String,     xxhash64,   XxHash64,
    std_hashset, HashSet<String8,BuildHasherDefault<XxHash64>>,     String8,    xxhash64,   XxHash64,
    std_hashset, HashSet<String16,BuildHasherDefault<XxHash64>>,    String16,   xxhash64,   XxHash64,
    std_hashset, HashSet<String32,BuildHasherDefault<XxHash64>>,    String32,   xxhash64,   XxHash64,
    std_hashset, HashSet<StringSlow,BuildHasherDefault<XxHash64>>,  StringSlow, xxhash64,   XxHash64
);
