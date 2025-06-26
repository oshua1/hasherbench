mod common;

use core::hash::Hasher;

use foldhash::fast::FixedState;
use foldhash::fast::FoldHasher;
use hashbrown::HashSet;

use common::String8;
use common::String16;
use common::String32;
use common::StringFmtDyn;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,FixedState>,             u32,            u32,            foldhasher, FoldHasher,
    hashbrown, HashSet<usize,FixedState>,           usize,          usize,          foldhasher, FoldHasher,
    hashbrown, HashSet<u128,FixedState>,            u128,           u128,           foldhasher, FoldHasher,
    hashbrown, HashSet<String,FixedState>,          String,         string,         foldhasher, FoldHasher,
    hashbrown, HashSet<String8,FixedState>,         String8,        string8,        foldhasher, FoldHasher,
    hashbrown, HashSet<String16,FixedState>,        String16,       string16,       foldhasher, FoldHasher,
    hashbrown, HashSet<String32,FixedState>,        String32,       string32,       foldhasher, FoldHasher,
    hashbrown, HashSet<StringFmtDyn,FixedState>,    StringFmtDyn,   stringfmtdyn,   foldhasher, FoldHasher
);

// Special implementation for hashbrown::HashSet and FoldHasher which doesn't implement Default

impl<KEY, HASHER> common::HashSetTrait<KEY, HASHER> for HashSet<KEY, FixedState>
where
    KEY: common::ProduceKey,
    HASHER: Hasher,
{
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, FixedState::default())
    }
    fn insert(&mut self, key: KEY) {
        Self::insert(self, key);
    }
    fn get<'a: 'b, 'b>(&'a self, key: &'b KEY) -> Option<&'b KEY> {
        Self::get(self, key)
    }
    fn shrink_to_fit(&mut self) -> &mut Self {
        Self::shrink_to_fit(self);
        self
    }
    fn len(&self) -> usize {
        Self::len(self)
    }
}
