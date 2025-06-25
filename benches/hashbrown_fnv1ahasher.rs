mod common;

use hashbrown::HashSet;
use hashers::fnv::FNV1aHasher64;
use common::ProduceKey;

create_benchmark! (hashbrown, fnv1a64, FNV1aHasher64);
