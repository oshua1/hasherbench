mod common;

use hashers::jenkins::spooky_hash::SpookyHasher;
use hashbrown::HashSet;
use common::ProduceKey;

create_benchmark! (hashbrown, spookyhasher, SpookyHasher);
