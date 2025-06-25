mod common;

use std::collections::HashSet;
use farmhash::FarmHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, farmhasher, FarmHasher);
