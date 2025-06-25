mod common;

use std::collections::HashSet;
use hashers::jenkins::OAATHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, oaathasher, OAATHasher);
