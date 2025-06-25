mod common;

use hashbrown::HashSet;
use seahash::SeaHasher;
use common::ProduceKey;

create_benchmark! (hashbrown, seahasher, SeaHasher);
