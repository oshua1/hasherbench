mod common;

use std::collections::HashSet;
use farmhash::FarmHasher;

create_benchmark! (std_hashset, farmhasher, FarmHasher);
