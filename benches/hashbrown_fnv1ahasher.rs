#![feature(random)]

mod common;

use hashbrown::HashSet;
use hashers::fnv::FNV1aHasher64;

create_benchmark! (hashbrown, fnv1a64, FNV1aHasher64);
