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

use highhash::city::CityHasher128;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(std_hashset);

create_benchmark! (std_hashset,
    std_hashset, HashSet<u32,BuildHasherDefault<CityHasher128>>,        u32,        cityhasher128,  CityHasher128,
    std_hashset, HashSet<usize,BuildHasherDefault<CityHasher128>>,      usize,      cityhasher128,  CityHasher128,
    std_hashset, HashSet<u128,BuildHasherDefault<CityHasher128>>,       u128,       cityhasher128,  CityHasher128,
    std_hashset, HashSet<String,BuildHasherDefault<CityHasher128>>,     String,     cityhasher128,  CityHasher128,
    std_hashset, HashSet<String8,BuildHasherDefault<CityHasher128>>,    String8,    cityhasher128,  CityHasher128,
    std_hashset, HashSet<String16,BuildHasherDefault<CityHasher128>>,   String16,   cityhasher128,  CityHasher128,
    std_hashset, HashSet<String32,BuildHasherDefault<CityHasher128>>,   String32,   cityhasher128,  CityHasher128,
    std_hashset, HashSet<StringSlow,BuildHasherDefault<CityHasher128>>, StringSlow, cityhasher128,  CityHasher128
);
