mod common;

use std::collections::HashSet;
use hashers::oz::SDBMHasher;

create_benchmark! (std_hashset, sdbmhasher, SDBMHasher);
