mod common;

use std::collections::HashSet;
use twox_hash::XxHash32;
use common::ProduceKey;

create_benchmark! (std_hashset, xxhash32, XxHash32);
