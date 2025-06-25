#![allow(
    single_use_lifetimes,
    reason = "impl HashSetTrait for vector_map_VecSet::get() fails without (otherwise unneeded) lifetime annotation"
)]
#![allow(
    dead_code,
    reason = " is_empty() is never used yet should be defined besides len()"
)]

use core::hash::BuildHasherDefault;
use core::hash::Hasher;

use foldhash::fast::FixedState;

use crate::common::ProduceKey;

/// Must be implemented for all `HashSet` / `HashMap` types to be benchmarkable here
pub trait HashSetTrait<KEY: ProduceKey, HASHER: Hasher> {
    fn with_capacity(capacity: usize) -> Self;
    fn insert(&mut self, key: KEY);
    fn get<'a: 'b, 'b>(&'a self, key: &'b KEY) -> Option<&'b KEY>;
    fn shrink_to_fit(&mut self) -> &mut Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// Standard implementation for std HashSet and most Hashers; particularly those implementing Default

impl<KEY, HASHER> HashSetTrait<KEY, HASHER> for std::collections::HashSet<KEY, BuildHasherDefault<HASHER>>
where
    KEY: ProduceKey,
    HASHER: Hasher + Default,
{
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildHasherDefault::<HASHER>::new())
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

// Standard implementation for Hashbrown HashSet and most Hashers; particularly those implementing Default

impl<KEY, HASHER> HashSetTrait<KEY, HASHER> for hashbrown::HashSet<KEY, BuildHasherDefault<HASHER>>
where
    KEY: ProduceKey,
    HASHER: Hasher + Default,
{
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildHasherDefault::<HASHER>::new())
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
