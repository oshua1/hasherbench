mod common;

use std::collections::HashSet;
use ahash::AHasher;

create_benchmark! (std_hashset, ahasher, AHasher);
