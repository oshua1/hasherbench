#![feature(random)]

mod common;

use ahash::AHasher;
use hashbrown::HashSet;

create_benchmark! (hashbrown, ahasher, AHasher);
