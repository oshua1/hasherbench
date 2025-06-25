mod common;

use std::collections::HashSet;
use hashers::fnv::FNV1aHasher64;
use common::ProduceKey;

create_benchmark! (std_hashset, fnv1a64, FNV1aHasher64);
