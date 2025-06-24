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

use core::hash::SipHasher;

use common::HashSetTrait;
use common::ProduceKey;
use common::String16;
use common::StringSlow;

criterion::criterion_main!(litemap);

create_benchmark! (litemap,
    // Hasher is not used by litemap::LiteMap
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 10, 100,
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 100, 100,
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        sip_hash, SipHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 10, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 100, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      sip_hash, SipHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 10, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 100, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       sip_hash, SipHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 10, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 100, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     String,     sip_hash, SipHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 10, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 100, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   String16,   sip_hash, SipHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 10, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 100, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, StringSlow, sip_hash, SipHasher, 100000, 100);

impl<KEY: ProduceKey> HashSetTrait<KEY, SipHasher> for litemap::LiteMap<KEY, ()> {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
    #[inline]
    fn insert(&mut self, key: KEY) {
        let _ = self.insert(key, ());
    }
    #[inline]
    fn get<'a: 'b, 'b>(&'a self, key: &'b KEY) -> Option<&'b KEY> {
        self.get(key).map(|()| key)
    }
    #[inline]
    fn shrink_to_fit(&mut self) -> &mut Self {
        self
    }
    #[inline]
    fn len(&self) -> usize {
        Self::len(self)
    }
}
