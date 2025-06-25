mod common;

use std::collections::HashSet;
use twox_hash::XxHash3_64;
use common::ProduceKey;

create_benchmark! (std_hashset, xxhash3_64, XxHash3_64);
