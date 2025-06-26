#![feature(random)]

mod common;

use hashbrown::HashSet;
use zwohash::ZwoHasher;

create_benchmark! (hashbrown, zwohasher, ZwoHasher);
