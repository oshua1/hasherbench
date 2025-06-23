#![allow(deprecated, reason = "Just benchmarking")]
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
use core::hash::SipHasher;

use hashbrown::HashSet;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<SipHasher>>,          u32,        sip_hash,   SipHasher,
    hashbrown, HashSet<usize,BuildHasherDefault<SipHasher>>,        usize,      sip_hash,   SipHasher,
    hashbrown, HashSet<u128,BuildHasherDefault<SipHasher>>,         u128,       sip_hash,   SipHasher,
    hashbrown, HashSet<String,BuildHasherDefault<SipHasher>>,       String,     sip_hash,   SipHasher,
    hashbrown, HashSet<String8,BuildHasherDefault<SipHasher>>,      String8,    sip_hash,   SipHasher,
    hashbrown, HashSet<String16,BuildHasherDefault<SipHasher>>,     String16,   sip_hash,   SipHasher,
    hashbrown, HashSet<String32,BuildHasherDefault<SipHasher>>,     String32,   sip_hash,   SipHasher,
    hashbrown, HashSet<StringSlow,BuildHasherDefault<SipHasher>>,   StringSlow, sip_hash,   SipHasher
);
