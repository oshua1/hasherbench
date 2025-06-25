mod common;

use hashbrown::HashSet;
use hashers::oz::SDBMHasher;

create_benchmark! (hashbrown, sdbmhasher, SDBMHasher);
