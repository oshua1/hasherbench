#![feature(random)]

mod common;

use std::hash::DefaultHasher;
use common::HashSetTrait;
use common::ProduceKey;
use common::String16;
use common::String128;
use common::StringFmtDyn;

criterion::criterion_main!(vecmap);

create_benchmark! (vecmap,
    // Hasher is not used by vecmap::VecSet
    setup vecmap_vecset,    vecmap::VecSet<u32>,            u32,            u32,            default_hasher, DefaultHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<u32>,            u32,            u32,            default_hasher, DefaultHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<u32>,            u32,            u32,            default_hasher, DefaultHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<u32>,            u32,            u32,            default_hasher, DefaultHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u32>,            u32,            u32,            default_hasher, DefaultHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u32>,            u32,            u32,            default_hasher, DefaultHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u32>,            u32,            u32,            default_hasher, DefaultHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u32>,            u32,            u32,            default_hasher, DefaultHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<usize>,          usize,          usize,          default_hasher, DefaultHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<usize>,          usize,          usize,          default_hasher, DefaultHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<usize>,          usize,          usize,          default_hasher, DefaultHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<usize>,          usize,          usize,          default_hasher, DefaultHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<usize>,          usize,          usize,          default_hasher, DefaultHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<usize>,          usize,          usize,          default_hasher, DefaultHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<usize>,          usize,          usize,          default_hasher, DefaultHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<usize>,          usize,          usize,          default_hasher, DefaultHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<u128>,           u128,           u128,           default_hasher, DefaultHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<u128>,           u128,           u128,           default_hasher, DefaultHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<u128>,           u128,           u128,           default_hasher, DefaultHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<u128>,           u128,           u128,           default_hasher, DefaultHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u128>,           u128,           u128,           default_hasher, DefaultHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u128>,           u128,           u128,           default_hasher, DefaultHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u128>,           u128,           u128,           default_hasher, DefaultHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<u128>,           u128,           u128,           default_hasher, DefaultHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String>,         String,         string,         default_hasher, DefaultHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<String>,         String,         string,         default_hasher, DefaultHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<String>,         String,         string,         default_hasher, DefaultHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String>,         String,         string,         default_hasher, DefaultHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String>,         String,         string,         default_hasher, DefaultHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String>,         String,         string,         default_hasher, DefaultHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String>,         String,         string,         default_hasher, DefaultHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String>,         String,         string,         default_hasher, DefaultHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String16>,       String16,       string16,       default_hasher, DefaultHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<String16>,       String16,       string16,       default_hasher, DefaultHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<String16>,       String16,       string16,       default_hasher, DefaultHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String16>,       String16,       string16,       default_hasher, DefaultHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String16>,       String16,       string16,       default_hasher, DefaultHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String16>,       String16,       string16,       default_hasher, DefaultHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String16>,       String16,       string16,       default_hasher, DefaultHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String16>,       String16,       string16,       default_hasher, DefaultHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String128>,      String128,      string128,      default_hasher, DefaultHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<String128>,      String128,      string128,      default_hasher, DefaultHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<String128>,      String128,      string128,      default_hasher, DefaultHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<String128>,      String128,      string128,      default_hasher, DefaultHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String128>,      String128,      string128,      default_hasher, DefaultHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String128>,      String128,      string128,      default_hasher, DefaultHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String128>,      String128,      string128,      default_hasher, DefaultHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<String128>,      String128,      string128,      default_hasher, DefaultHasher, 10000, 100,
    setup vecmap_vecset,    vecmap::VecSet<StringFmtDyn>,   StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher, 10, 100,
    setup vecmap_vecset,    vecmap::VecSet<StringFmtDyn>,   StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher, 100, 100,
    setup vecmap_vecset,    vecmap::VecSet<StringFmtDyn>,   StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher, 1000, 100,
    setup vecmap_vecset,    vecmap::VecSet<StringFmtDyn>,   StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher, 10000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<StringFmtDyn>,   StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher, 10, 100,
    lookup vecmap_vecset,   vecmap::VecSet<StringFmtDyn>,   StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher, 100, 100,
    lookup vecmap_vecset,   vecmap::VecSet<StringFmtDyn>,   StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher, 1000, 100,
    lookup vecmap_vecset,   vecmap::VecSet<StringFmtDyn>,   StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher, 10000, 100);

impl<KEY: ProduceKey> HashSetTrait<KEY, DefaultHasher> for vecmap::VecSet<KEY> {
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
