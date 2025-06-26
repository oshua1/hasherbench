#![feature(random)]

mod common;

use hashbrown::HashSet;
use highway::HighwayHasher;

create_benchmark! (hashbrown, highwayhasher, HighwayHasher);
