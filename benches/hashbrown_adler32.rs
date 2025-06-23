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

use adler2::Adler32;
use hashbrown::HashSet;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<Adler32>>,        u32,        adler32,    Adler32,
    hashbrown, HashSet<usize,BuildHasherDefault<Adler32>>,      usize,      adler32,    Adler32,
    hashbrown, HashSet<u128,BuildHasherDefault<Adler32>>,       u128,       adler32,    Adler32,
    hashbrown, HashSet<String,BuildHasherDefault<Adler32>>,     String,     adler32,    Adler32,
    hashbrown, HashSet<String8,BuildHasherDefault<Adler32>>,    String8,    adler32,    Adler32,
    hashbrown, HashSet<String16,BuildHasherDefault<Adler32>>,   String16,   adler32,    Adler32,
    hashbrown, HashSet<String32,BuildHasherDefault<Adler32>>,   String32,   adler32,    Adler32,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<Adler32>>, StringSlow, adler32,    Adler32
);
