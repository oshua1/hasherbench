mod common;

use std::collections::HashSet;
use twox_hash::XxHash64;

create_benchmark! (std_hashset, xxhash64, XxHash64);
