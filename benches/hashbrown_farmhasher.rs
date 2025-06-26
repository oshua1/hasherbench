#![feature(random)]

mod common;

use std::collections::HashSet;
use farmhash::FarmHasher;

create_benchmark! (hashbrown, farmhasher, FarmHasher);
