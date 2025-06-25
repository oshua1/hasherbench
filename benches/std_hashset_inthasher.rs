mod common;

use core::hash::BuildHasherDefault;
use std::collections::HashSet;
use integer_hasher::IntHasher;

criterion::criterion_main!(std_hashset);

create_benchmark! (std_hashset,
    std_hashset, HashSet<u32,BuildHasherDefault<IntHasher<u32>>>,   u32,u32,integer_hasher, IntHasher<u32>,
    std_hashset, HashSet<u64,BuildHasherDefault<IntHasher<u64>>>,   u64,u64,integer_hasher, IntHasher<u64>
);
