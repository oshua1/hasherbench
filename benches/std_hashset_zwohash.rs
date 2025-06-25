mod common;

use std::collections::HashSet;
use zwohash::ZwoHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, zwohasher, ZwoHasher);
