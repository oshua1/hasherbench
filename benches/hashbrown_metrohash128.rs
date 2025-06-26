#![feature(random)]

mod common;

use hashbrown::HashSet;
use metrohash::MetroHash128;

create_benchmark! (hashbrown, metrohash128, MetroHash128);
