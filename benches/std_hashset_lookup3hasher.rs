mod common;

use std::collections::HashSet;
use hashers::jenkins::Lookup3Hasher;
use common::ProduceKey;

create_benchmark! (std_hashset, lookup3hasher, Lookup3Hasher);
