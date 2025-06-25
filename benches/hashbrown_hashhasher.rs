mod common;

use hashbrown::HashSet;
use hash_hasher::HashHasher;
use common::ProduceKey;

create_benchmark! (hashbrown, hash_hasher, HashHasher);
