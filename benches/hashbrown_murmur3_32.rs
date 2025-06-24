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
use highhash::murmur::Murmur3Hasher32;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<Murmur3Hasher32>>,        u32,        murmur3_32, Murmur3Hasher32,
    hashbrown, HashSet<usize,BuildHasherDefault<Murmur3Hasher32>>,      usize,      murmur3_32, Murmur3Hasher32,
    hashbrown, HashSet<u128,BuildHasherDefault<Murmur3Hasher32>>,       u128,       murmur3_32, Murmur3Hasher32,
    hashbrown, HashSet<String,BuildHasherDefault<Murmur3Hasher32>>,     String,     murmur3_32, Murmur3Hasher32,
    hashbrown, HashSet<String8,BuildHasherDefault<Murmur3Hasher32>>,    String8,    murmur3_32, Murmur3Hasher32,
    hashbrown, HashSet<String16,BuildHasherDefault<Murmur3Hasher32>>,   String16,   murmur3_32, Murmur3Hasher32,
    hashbrown, HashSet<String32,BuildHasherDefault<Murmur3Hasher32>>,   String32,   murmur3_32, Murmur3Hasher32,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<Murmur3Hasher32>>, StringSlow, murmur3_32, Murmur3Hasher32
);
