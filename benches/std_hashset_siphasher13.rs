#![feature (hashmap_internals)]
#![allow(internal_features, deprecated, reason = "Just benchmarking")]
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
use core::hash::SipHasher13;
use std::collections::HashSet;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

criterion::criterion_main!(std_hashset);

create_benchmark! (std_hashset,
    std_hashset, HashSet<u32,BuildHasherDefault<SipHasher13>>,      u32,        sip_hash13,     SipHasher13,
    std_hashset, HashSet<usize,BuildHasherDefault<SipHasher13>>,    usize,      sip_hash13,     SipHasher13,
    std_hashset, HashSet<u128,BuildHasherDefault<SipHasher13>>,     u128,       sip_hash13,     SipHasher13,
    std_hashset, HashSet<String,BuildHasherDefault<SipHasher13>>,   String,     sip_hash13,     SipHasher13,
    std_hashset, HashSet<String8,BuildHasherDefault<SipHasher13>>,  String8,    sip_hash13,     SipHasher13,
    std_hashset, HashSet<String16,BuildHasherDefault<SipHasher13>>, String16,   sip_hash13,     SipHasher13,
    std_hashset, HashSet<String32,BuildHasherDefault<SipHasher13>>, String32,   sip_hash13,     SipHasher13,
    std_hashset, HashSet<StringSlow,BuildHasherDefault<SipHasher13>>,StringSlow,sip_hash13,     SipHasher13
);
