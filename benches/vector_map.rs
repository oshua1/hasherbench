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

mod common;

use core::hash::SipHasher;

use common::HashSetTrait;
use common::ProduceKey;
use common::String16;
use common::StringSlow;

criterion::criterion_main!(vector_map);

create_benchmark! (vector_map,
    // Hasher is not used by vector_map::VecSet
    setup vectormap_vecset, vector_map::set::VecSet<u32>,                   u32,        sip_hash, SipHasher, 10, 100,
    setup vectormap_vecset, vector_map::set::VecSet<u32>,                   u32,        sip_hash, SipHasher, 100, 100,
    setup vectormap_vecset, vector_map::set::VecSet<u32>,                   u32,        sip_hash, SipHasher, 1000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<u32>,                   u32,        sip_hash, SipHasher, 10000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<u32>,                   u32,        sip_hash, SipHasher, 10, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<u32>,                   u32,        sip_hash, SipHasher, 100, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<u32>,                   u32,        sip_hash, SipHasher, 1000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<u32>,                   u32,        sip_hash, SipHasher, 10000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<usize>,                 usize,      sip_hash, SipHasher, 10, 100,
    setup vectormap_vecset, vector_map::set::VecSet<usize>,                 usize,      sip_hash, SipHasher, 100, 100,
    setup vectormap_vecset, vector_map::set::VecSet<usize>,                 usize,      sip_hash, SipHasher, 1000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<usize>,                 usize,      sip_hash, SipHasher, 10000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<usize>,                 usize,      sip_hash, SipHasher, 10, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<usize>,                 usize,      sip_hash, SipHasher, 100, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<usize>,                 usize,      sip_hash, SipHasher, 1000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<usize>,                 usize,      sip_hash, SipHasher, 10000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<u128>,                  u128,       sip_hash, SipHasher, 10, 100,
    setup vectormap_vecset, vector_map::set::VecSet<u128>,                  u128,       sip_hash, SipHasher, 100, 100,
    setup vectormap_vecset, vector_map::set::VecSet<u128>,                  u128,       sip_hash, SipHasher, 1000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<u128>,                  u128,       sip_hash, SipHasher, 10000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<u128>,                  u128,       sip_hash, SipHasher, 10, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<u128>,                  u128,       sip_hash, SipHasher, 100, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<u128>,                  u128,       sip_hash, SipHasher, 1000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<u128>,                  u128,       sip_hash, SipHasher, 10000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<String>,                String,     sip_hash, SipHasher, 10, 100,
    setup vectormap_vecset, vector_map::set::VecSet<String>,                String,     sip_hash, SipHasher, 100, 100,
    setup vectormap_vecset, vector_map::set::VecSet<String>,                String,     sip_hash, SipHasher, 1000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<String>,                String,     sip_hash, SipHasher, 10000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<String>,                String,     sip_hash, SipHasher, 10, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<String>,                String,     sip_hash, SipHasher, 100, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<String>,                String,     sip_hash, SipHasher, 1000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<String>,                String,     sip_hash, SipHasher, 10000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<String16>,              String16,   sip_hash, SipHasher, 10, 100,
    setup vectormap_vecset, vector_map::set::VecSet<String16>,              String16,   sip_hash, SipHasher, 100, 100,
    setup vectormap_vecset, vector_map::set::VecSet<String16>,              String16,   sip_hash, SipHasher, 1000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<String16>,              String16,   sip_hash, SipHasher, 10000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<String16>,              String16,   sip_hash, SipHasher, 10, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<String16>,              String16,   sip_hash, SipHasher, 100, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<String16>,              String16,   sip_hash, SipHasher, 1000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<String16>,              String16,   sip_hash, SipHasher, 10000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<StringSlow>,            StringSlow, sip_hash, SipHasher, 10, 100,
    setup vectormap_vecset, vector_map::set::VecSet<StringSlow>,            StringSlow, sip_hash, SipHasher, 100, 100,
    setup vectormap_vecset, vector_map::set::VecSet<StringSlow>,            StringSlow, sip_hash, SipHasher, 1000, 100,
    setup vectormap_vecset, vector_map::set::VecSet<StringSlow>,            StringSlow, sip_hash, SipHasher, 10000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<StringSlow>,            StringSlow, sip_hash, SipHasher, 10, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<StringSlow>,            StringSlow, sip_hash, SipHasher, 100, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<StringSlow>,            StringSlow, sip_hash, SipHasher, 1000, 100,
    lookup vectormap_vecset,vector_map::set::VecSet<StringSlow>,            StringSlow, sip_hash, SipHasher, 10000, 100);

impl<KEY: ProduceKey> HashSetTrait<KEY, SipHasher> for vector_map::set::VecSet<KEY> {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
    #[inline]
    fn insert(&mut self, key: KEY) {
        let _ = self.insert(key);
    }
    #[inline]
    fn get<'a: 'b, 'b>(&'a self, key: &'b KEY) -> Option<&'b KEY> {
        self.contains(key).then_some(key)
    }
    #[inline]
    fn shrink_to_fit(&mut self) -> &mut Self {
        Self::shrink_to_fit(self);
        self
    }
    #[inline]
    fn len(&self) -> usize {
        Self::len(self)
    }
}
