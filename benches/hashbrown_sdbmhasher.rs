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
use hashers::oz::SDBMHasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<SDBMHasher>>,         u32,        sdbmhasher, SDBMHasher,
    hashbrown, HashSet<usize,BuildHasherDefault<SDBMHasher>>,       usize,      sdbmhasher, SDBMHasher,
    hashbrown, HashSet<u128,BuildHasherDefault<SDBMHasher>>,        u128,       sdbmhasher, SDBMHasher,
    hashbrown, HashSet<String,BuildHasherDefault<SDBMHasher>>,      String,     sdbmhasher, SDBMHasher,
    hashbrown, HashSet<String8,BuildHasherDefault<SDBMHasher>>,     String8,    sdbmhasher, SDBMHasher,
    hashbrown, HashSet<String16,BuildHasherDefault<SDBMHasher>>,    String16,   sdbmhasher, SDBMHasher,
    hashbrown, HashSet<String32,BuildHasherDefault<SDBMHasher>>,    String32,   sdbmhasher, SDBMHasher,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<SDBMHasher>>,  StringSlow, sdbmhasher, SDBMHasher
);
