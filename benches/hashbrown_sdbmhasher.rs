mod common;

use hashbrown::HashSet;
use hashers::oz::SDBMHasher;
use common::ProduceKey;

create_benchmark! (hashbrown, sdbmhasher, SDBMHasher);
