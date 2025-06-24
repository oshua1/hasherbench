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
use seahash::SeaHasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown,  HashSet<u32,BuildHasherDefault<SeaHasher>>,         u32,        seahasher,  SeaHasher,
    hashbrown,  HashSet<usize,BuildHasherDefault<SeaHasher>>,       usize,      seahasher,  SeaHasher,
    hashbrown,  HashSet<u128,BuildHasherDefault<SeaHasher>>,        u128,       seahasher,  SeaHasher,
    hashbrown,  HashSet<String,BuildHasherDefault<SeaHasher>>,      String,     seahasher,  SeaHasher,
    hashbrown,  HashSet<String8,BuildHasherDefault<SeaHasher>>,     String8,    seahasher,  SeaHasher,
    hashbrown,  HashSet<String16,BuildHasherDefault<SeaHasher>>,    String16,   seahasher,  SeaHasher,
    hashbrown,  HashSet<String32,BuildHasherDefault<SeaHasher>>,    String32,   seahasher,  SeaHasher,
    hashbrown,  HashSet<StringSlow,BuildHasherDefault<SeaHasher>>,  StringSlow, seahasher,  SeaHasher
);
