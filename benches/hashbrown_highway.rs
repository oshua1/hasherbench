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
use highway::HighwayHasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<HighwayHasher>>,          u32,        highwayhasher,  HighwayHasher,
    hashbrown, HashSet<usize,BuildHasherDefault<HighwayHasher>>,        usize,      highwayhasher,  HighwayHasher,
    hashbrown, HashSet<u128,BuildHasherDefault<HighwayHasher>>,         u128,       highwayhasher,  HighwayHasher,
    hashbrown, HashSet<String,BuildHasherDefault<HighwayHasher>>,       String,     highwayhasher,  HighwayHasher,
    hashbrown, HashSet<String8,BuildHasherDefault<HighwayHasher>>,      String8,    highwayhasher,  HighwayHasher,
    hashbrown, HashSet<String16,BuildHasherDefault<HighwayHasher>>,     String16,   highwayhasher,  HighwayHasher,
    hashbrown, HashSet<String32,BuildHasherDefault<HighwayHasher>>,     String32,   highwayhasher,  HighwayHasher,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<HighwayHasher>>,   StringSlow, highwayhasher,  HighwayHasher
);
