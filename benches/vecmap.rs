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

criterion::criterion_main!(vecmap);

create_benchmark! (vecmap,
    // Hasher is not used by vecmap::VecSet
    setup vecmap_vecset,    vecmap::VecSet<u32>,                            u32,        sip_hash, SipHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<u32>,                            u32,        sip_hash, SipHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<u32>,                            u32,        sip_hash, SipHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<u32>,                            u32,        sip_hash, SipHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u32>,                            u32,        sip_hash, SipHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u32>,                            u32,        sip_hash, SipHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u32>,                            u32,        sip_hash, SipHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u32>,                            u32,        sip_hash, SipHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<usize>,                          usize,      sip_hash, SipHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<usize>,                          usize,      sip_hash, SipHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<usize>,                          usize,      sip_hash, SipHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<usize>,                          usize,      sip_hash, SipHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<usize>,                          usize,      sip_hash, SipHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<usize>,                          usize,      sip_hash, SipHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<usize>,                          usize,      sip_hash, SipHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<usize>,                          usize,      sip_hash, SipHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<u128>,                           u128,       sip_hash, SipHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<u128>,                           u128,       sip_hash, SipHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<u128>,                           u128,       sip_hash, SipHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<u128>,                           u128,       sip_hash, SipHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u128>,                           u128,       sip_hash, SipHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u128>,                           u128,       sip_hash, SipHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u128>,                           u128,       sip_hash, SipHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u128>,                           u128,       sip_hash, SipHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String>,                         String,     sip_hash, SipHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<String>,                         String,     sip_hash, SipHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<String>,                         String,     sip_hash, SipHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String>,                         String,     sip_hash, SipHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String>,                         String,     sip_hash, SipHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String>,                         String,     sip_hash, SipHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String>,                         String,     sip_hash, SipHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String>,                         String,     sip_hash, SipHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String16>,                       String16,   sip_hash, SipHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<String16>,                       String16,   sip_hash, SipHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<String16>,                       String16,   sip_hash, SipHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String16>,                       String16,   sip_hash, SipHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String16>,                       String16,   sip_hash, SipHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String16>,                       String16,   sip_hash, SipHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String16>,                       String16,   sip_hash, SipHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String16>,                       String16,   sip_hash, SipHasher, 10000, 100);

impl<KEY: ProduceKey> HashSetTrait<KEY, SipHasher> for vecmap::VecSet<KEY> {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
    #[inline]
    fn insert(&mut self, key: KEY) {
        Self::insert(self, key);
    }
    #[inline]
    fn get<'a: 'b, 'b>(&'a self, key: &'b KEY) -> Option<&'b KEY> {
        Self::get(self, key)
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
