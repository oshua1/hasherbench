mod common;

use std::collections::HashSet;
use twox_hash::XxHash32;

create_benchmark! (std_hashset, xxhash32, XxHash32);
