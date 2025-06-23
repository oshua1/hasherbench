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
use hash_hasher::HashHasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<HashHasher>>,       u32,          hash_hasher,    HashHasher,
    hashbrown, HashSet<usize,BuildHasherDefault<HashHasher>>,     usize,        hash_hasher,    HashHasher,
    hashbrown, HashSet<u128,BuildHasherDefault<HashHasher>>,      u128,         hash_hasher,    HashHasher,
    hashbrown, HashSet<String,BuildHasherDefault<HashHasher>>,    String,       hash_hasher,    HashHasher,
    hashbrown, HashSet<String8,BuildHasherDefault<HashHasher>>,   String8,      hash_hasher,    HashHasher,
    hashbrown, HashSet<String16,BuildHasherDefault<HashHasher>>,  String16,     hash_hasher,    HashHasher,
    hashbrown, HashSet<String32,BuildHasherDefault<HashHasher>>,  String32,     hash_hasher,    HashHasher,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<HashHasher>>,StringSlow,   hash_hasher,    HashHasher
);
