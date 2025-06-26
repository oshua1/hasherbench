#![feature(random)]

mod common;

use std::collections::HashSet;
use twox_hash::XxHash3_64;

create_benchmark! (std_hashset, xxhash3_64, XxHash3_64);
