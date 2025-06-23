#![allow(internal_features, deprecated, reason = "Just benchmarking")]
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
use zwohash::ZwoHasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<ZwoHasher>>,          u32,        zwohasher,  ZwoHasher,
    hashbrown, HashSet<usize,BuildHasherDefault<ZwoHasher>>,        usize,      zwohasher,  ZwoHasher,
    hashbrown, HashSet<u128,BuildHasherDefault<ZwoHasher>>,         u128,       zwohasher,  ZwoHasher,
    hashbrown, HashSet<String,BuildHasherDefault<ZwoHasher>>,       String,     zwohasher,  ZwoHasher,
    hashbrown, HashSet<String8,BuildHasherDefault<ZwoHasher>>,      String8,    zwohasher,  ZwoHasher,
    hashbrown, HashSet<String16,BuildHasherDefault<ZwoHasher>>,     String16,   zwohasher,  ZwoHasher,
    hashbrown, HashSet<String32,BuildHasherDefault<ZwoHasher>>,     String32,   zwohasher,  ZwoHasher,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<ZwoHasher>>,   StringSlow, zwohasher,  ZwoHasher
);
