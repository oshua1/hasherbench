mod common;

use std::collections::HashSet;
use hashers::jenkins::spooky_hash::SpookyHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, spookyhasher, SpookyHasher);
