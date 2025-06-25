mod common;

use core::hash::BuildHasherDefault;
use hashbrown::HashSet;
use integer_hasher::IntHasher;

criterion::criterion_main!(hashbrown);

create_benchmark! (hashbrown,
    hashbrown, HashSet<u32,BuildHasherDefault<IntHasher<u32>>>, u32,u32,integer_hasher, IntHasher<u32>,
    hashbrown, HashSet<u64,BuildHasherDefault<IntHasher<u64>>>, u64,u64,integer_hasher, IntHasher<u64>
);
