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
use hashers::pigeon::Bricolage;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<Bricolage>>,          u32,        bricolage,  Bricolage,
    hashbrown, HashSet<usize,BuildHasherDefault<Bricolage>>,        usize,      bricolage,  Bricolage,
    hashbrown, HashSet<u128,BuildHasherDefault<Bricolage>>,         u128,       bricolage,  Bricolage,
    hashbrown, HashSet<String,BuildHasherDefault<Bricolage>>,       String,     bricolage,  Bricolage,
    hashbrown, HashSet<String8,BuildHasherDefault<Bricolage>>,      String8,    bricolage,  Bricolage,
    hashbrown, HashSet<String16,BuildHasherDefault<Bricolage>>,     String16,   bricolage,  Bricolage,
    hashbrown, HashSet<String32,BuildHasherDefault<Bricolage>>,     String32,   bricolage,  Bricolage,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<Bricolage>>,   StringSlow, bricolage,  Bricolage
);
