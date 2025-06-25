mod common;

use std::collections::HashSet;
use seahash::SeaHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, seahasher, SeaHasher);
