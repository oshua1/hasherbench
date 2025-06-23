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

use hashers::jenkins::spooky_hash::SpookyHasher;
use hashbrown::HashSet;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<SpookyHasher>>,           u32,        spookyhasher,   SpookyHasher,
    hashbrown, HashSet<usize,BuildHasherDefault<SpookyHasher>>,         usize,      spookyhasher,   SpookyHasher,
    hashbrown, HashSet<u128,BuildHasherDefault<SpookyHasher>>,          u128,       spookyhasher,   SpookyHasher,
    hashbrown, HashSet<String,BuildHasherDefault<SpookyHasher>>,        String,     spookyhasher,   SpookyHasher,
    hashbrown, HashSet<String8,BuildHasherDefault<SpookyHasher>>,       String8,    spookyhasher,   SpookyHasher,
    hashbrown, HashSet<String16,BuildHasherDefault<SpookyHasher>>,      String16,   spookyhasher,   SpookyHasher,
    hashbrown, HashSet<String32,BuildHasherDefault<SpookyHasher>>,      String32,   spookyhasher,   SpookyHasher,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<SpookyHasher>>,    StringSlow, spookyhasher,   SpookyHasher
);
