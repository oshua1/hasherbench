mod common;

use std::collections::HashSet;
use farmhash::FarmHasher;
use common::ProduceKey;

create_benchmark! (hashbrown, farmhasher, FarmHasher);
