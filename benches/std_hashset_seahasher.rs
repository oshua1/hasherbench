mod common;

use std::collections::HashSet;
use seahash::SeaHasher;

create_benchmark! (std_hashset, seahasher, SeaHasher);
