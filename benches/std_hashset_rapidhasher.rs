mod common;

use std::collections::HashSet;
use rapidhash::RapidHasher;

create_benchmark! (std_hashset, rapidhasher, RapidHasher);
