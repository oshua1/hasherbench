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
use twox_hash::XxHash3_64;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<XxHash3_64>>,         u32,        xxhash3_64, XxHash3_64,
    hashbrown, HashSet<usize,BuildHasherDefault<XxHash3_64>>,       usize,      xxhash3_64, XxHash3_64,
    hashbrown, HashSet<u128,BuildHasherDefault<XxHash3_64>>,        u128,       xxhash3_64, XxHash3_64,
    hashbrown, HashSet<String,BuildHasherDefault<XxHash3_64>>,      String,     xxhash3_64, XxHash3_64,
    hashbrown, HashSet<String8,BuildHasherDefault<XxHash3_64>>,     String8,    xxhash3_64, XxHash3_64,
    hashbrown, HashSet<String16,BuildHasherDefault<XxHash3_64>>,    String16,   xxhash3_64, XxHash3_64,
    hashbrown, HashSet<String32,BuildHasherDefault<XxHash3_64>>,    String32,   xxhash3_64, XxHash3_64,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<XxHash3_64>>,  StringSlow, xxhash3_64, XxHash3_64
);
