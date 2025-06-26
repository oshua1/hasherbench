#![feature(random)]

mod common;

use std::collections::HashSet;
use highhash::murmur::Murmur3Hasher32;

create_benchmark! (std_hashset, murmur3_32, Murmur3Hasher32);
