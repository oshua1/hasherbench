mod common;

use std::collections::HashSet;
use zwohash::ZwoHasher;

create_benchmark! (std_hashset, zwohasher, ZwoHasher);
