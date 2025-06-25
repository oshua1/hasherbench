mod common;

use std::collections::HashSet;
use rapidhash::RapidHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, rapidhasher, RapidHasher);
