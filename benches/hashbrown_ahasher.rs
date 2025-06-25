mod common;

use ahash::AHasher;
use hashbrown::HashSet;
use common::ProduceKey;

create_benchmark! (hashbrown, ahasher, AHasher);
