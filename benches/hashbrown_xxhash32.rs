#![feature(random)]

mod common;

use hashbrown::HashSet;
use twox_hash::XxHash32;

create_benchmark! (hashbrown, xxhash32, XxHash32);
