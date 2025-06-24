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
use std::collections::HashSet;

use highway::HighwayHasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(std_hashset);

create_benchmark! (std_hashset,
    std_hashset, HashSet<u32,BuildHasherDefault<HighwayHasher>>,        u32,        highwayhasher,  HighwayHasher,
    std_hashset, HashSet<usize,BuildHasherDefault<HighwayHasher>>,      usize,      highwayhasher,  HighwayHasher,
    std_hashset, HashSet<u128,BuildHasherDefault<HighwayHasher>>,       u128,       highwayhasher,  HighwayHasher,
    std_hashset, HashSet<String,BuildHasherDefault<HighwayHasher>>,     String,     highwayhasher,  HighwayHasher,
    std_hashset, HashSet<String8,BuildHasherDefault<HighwayHasher>>,    String8,    highwayhasher,  HighwayHasher,
    std_hashset, HashSet<String16,BuildHasherDefault<HighwayHasher>>,   String16,   highwayhasher,  HighwayHasher,
    std_hashset, HashSet<String32,BuildHasherDefault<HighwayHasher>>,   String32,   highwayhasher,  HighwayHasher,
    std_hashset, HashSet<StringSlow,BuildHasherDefault<HighwayHasher>>, StringSlow, highwayhasher,  HighwayHasher
);
