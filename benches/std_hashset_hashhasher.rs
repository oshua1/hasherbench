#![feature(random)]

mod common;

use std::collections::HashSet;
use hash_hasher::HashHasher;

create_benchmark! (std_hashset, hash_hasher, HashHasher);
