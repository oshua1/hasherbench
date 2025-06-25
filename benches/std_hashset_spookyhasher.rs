mod common;

use std::collections::HashSet;
use hashers::jenkins::spooky_hash::SpookyHasher;

create_benchmark! (std_hashset, spookyhasher, SpookyHasher);
