#![feature(random)]

mod common;

use hashbrown::HashSet;
use hash_hasher::HashHasher;

create_benchmark! (hashbrown, hash_hasher, HashHasher);
