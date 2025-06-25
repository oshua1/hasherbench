mod common;

use std::collections::HashSet;
use ahash::AHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, ahasher, AHasher);
