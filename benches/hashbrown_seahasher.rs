#![feature(random)]

mod common;

use hashbrown::HashSet;
use seahash::SeaHasher;

create_benchmark! (hashbrown, seahasher, SeaHasher);
