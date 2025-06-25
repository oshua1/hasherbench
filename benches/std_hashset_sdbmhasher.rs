mod common;

use std::collections::HashSet;
use hashers::oz::SDBMHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, sdbmhasher, SDBMHasher);
