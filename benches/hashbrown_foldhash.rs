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

use core::hash::Hasher;

use foldhash::fast::FixedState;
use foldhash::fast::FoldHasher;
use hashbrown::HashSet;

use common::ProduceKey;
use common::String8;
use common::String16;
use common::String32;
use common::StringSlow;

use crate::common::HashSetTrait;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,FixedState>,         u32,        foldhasher, FoldHasher,
    hashbrown, HashSet<usize,FixedState>,       usize,      foldhasher, FoldHasher,
    hashbrown, HashSet<u128,FixedState>,        u128,       foldhasher, FoldHasher,
    hashbrown, HashSet<String,FixedState>,      String,     foldhasher, FoldHasher,
    hashbrown, HashSet<String8,FixedState>,     String8,    foldhasher, FoldHasher,
    hashbrown, HashSet<String16,FixedState>,    String16,   foldhasher, FoldHasher,
    hashbrown, HashSet<String32,FixedState>,    String32,   foldhasher, FoldHasher,
    hashbrown, HashSet<StringSlow,FixedState>,  StringSlow, foldhasher, FoldHasher
);

// Special implementation for hashbrown::HashSet and FoldHasher which doesn't implement Default

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
