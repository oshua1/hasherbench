#![feature(random)]

mod common;

use hashbrown::HashSet;
use highhash::murmur::Murmur3Hasher128x64;

create_benchmark! (hashbrown, murmur3_128x64, Murmur3Hasher128x64);
