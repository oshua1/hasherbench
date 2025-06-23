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
use hashers::jenkins::Lookup3Hasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<Lookup3Hasher>>,          u32,        lookup3hasher,  Lookup3Hasher,
    hashbrown, HashSet<usize,BuildHasherDefault<Lookup3Hasher>>,        usize,      lookup3hasher,  Lookup3Hasher,
    hashbrown, HashSet<u128,BuildHasherDefault<Lookup3Hasher>>,         u128,       lookup3hasher,  Lookup3Hasher,
    hashbrown, HashSet<String,BuildHasherDefault<Lookup3Hasher>>,       String,     lookup3hasher,  Lookup3Hasher,
    hashbrown, HashSet<String8,BuildHasherDefault<Lookup3Hasher>>,      String8,    lookup3hasher,  Lookup3Hasher,
    hashbrown, HashSet<String16,BuildHasherDefault<Lookup3Hasher>>,     String16,   lookup3hasher,  Lookup3Hasher,
    hashbrown, HashSet<String32,BuildHasherDefault<Lookup3Hasher>>,     String32,   lookup3hasher,  Lookup3Hasher,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<Lookup3Hasher>>,   StringSlow, lookup3hasher,  Lookup3Hasher
);
