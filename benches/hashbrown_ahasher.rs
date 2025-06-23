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

use ahash::AHasher;
use hashbrown::HashSet;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<AHasher>>,        u32,        ahasher,    AHasher,
    hashbrown, HashSet<usize,BuildHasherDefault<AHasher>>,      usize,      ahasher,    AHasher,
    hashbrown, HashSet<u128,BuildHasherDefault<AHasher>>,       u128,       ahasher,    AHasher,
    hashbrown, HashSet<String,BuildHasherDefault<AHasher>>,     String,     ahasher,    AHasher,
    hashbrown, HashSet<String8,BuildHasherDefault<AHasher>>,    String8,    ahasher,    AHasher,
    hashbrown, HashSet<String16,BuildHasherDefault<AHasher>>,   String16,   ahasher,    AHasher,
    hashbrown, HashSet<String32,BuildHasherDefault<AHasher>>,   String32,   ahasher,    AHasher,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<AHasher>>, StringSlow, ahasher,    AHasher
);
