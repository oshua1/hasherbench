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
use twox_hash::XxHash32;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<XxHash32>>,       u32,        xxhash32,   XxHash32,
    hashbrown, HashSet<usize,BuildHasherDefault<XxHash32>>,     usize,      xxhash32,   XxHash32,
    hashbrown, HashSet<u128,BuildHasherDefault<XxHash32>>,      u128,       xxhash32,   XxHash32,
    hashbrown, HashSet<String,BuildHasherDefault<XxHash32>>,    String,     xxhash32,   XxHash32,
    hashbrown, HashSet<String8,BuildHasherDefault<XxHash32>>,   String8,    xxhash32,   XxHash32,
    hashbrown, HashSet<String16,BuildHasherDefault<XxHash32>>,  String16,   xxhash32,   XxHash32,
    hashbrown, HashSet<String32,BuildHasherDefault<XxHash32>>,  String32,   xxhash32,   XxHash32,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<XxHash32>>,StringSlow, xxhash32,   XxHash32
);
