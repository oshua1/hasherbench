mod common;

use core::hash::Hasher;
use std::collections::HashSet;

use foldhash::fast::FixedState;
use foldhash::fast::FoldHasher;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

use crate::common::HashSetTrait;

criterion::criterion_main!(std_hashset);

create_benchmark! (std_hashset,
    std_hashset, HashSet<u32,FixedState>,           u32,        u32,        foldhasher,     FoldHasher,
    std_hashset, HashSet<usize,FixedState>,         usize,      usize,      foldhasher,     FoldHasher,
    std_hashset, HashSet<u128,FixedState>,          u128,       u128,       foldhasher,     FoldHasher,
    std_hashset, HashSet<String,FixedState>,        String,     string,     foldhasher,     FoldHasher,
    std_hashset, HashSet<String8,FixedState>,       String8,    string8,    foldhasher,     FoldHasher,
    std_hashset, HashSet<String16,FixedState>,      String16,   string16,   foldhasher,     FoldHasher,
    std_hashset, HashSet<String32,FixedState>,      String32,   string32,   foldhasher,     FoldHasher,
    std_hashset, HashSet<StringSlow,FixedState>,    StringSlow, stringslow, foldhasher,     FoldHasher
);

// Special implementation for std HashSet and FoldHasher which doesn't implement Default

impl<KEY, HASHER> HashSetTrait<KEY, HASHER> for HashSet<KEY, FixedState>
where
    KEY: ProduceKey,
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
