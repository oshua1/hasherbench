mod common;

use hashbrown::HashSet;
use hashers::oz::DJB2Hasher;
use common::ProduceKey;

create_benchmark! (hashbrown, djb2hasher, DJB2Hasher);
