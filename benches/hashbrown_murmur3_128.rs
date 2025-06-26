#![feature(random)]

mod common;

use hashbrown::HashSet;
use highhash::murmur::Murmur3Hasher128;

create_benchmark! (hashbrown, murmur3_128, Murmur3Hasher128);
