mod common;

use std::collections::HashSet;
use hashers::oz::DJB2Hasher;
use common::ProduceKey;

create_benchmark! (std_hashset, djb2hasher, DJB2Hasher);
