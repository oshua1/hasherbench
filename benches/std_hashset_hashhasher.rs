mod common;

use std::collections::HashSet;
use hash_hasher::HashHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, hash_hasher, HashHasher);
