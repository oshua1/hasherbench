mod common;

use hashbrown::HashSet;
use rapidhash::RapidHasher;
use common::ProduceKey;

create_benchmark! (hashbrown, rapidhasher, RapidHasher);
