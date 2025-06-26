#![feature(random)]

mod common;

use hashbrown::HashSet;
use rapidhash::RapidHasher;

create_benchmark! (hashbrown, rapidhasher, RapidHasher);
