#![feature(random)]

mod common;

use std::collections::HashSet;
use highway::HighwayHasher;

create_benchmark! (std_hashset, highwayhasher, HighwayHasher);
