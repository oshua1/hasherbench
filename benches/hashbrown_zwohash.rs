mod common;

use hashbrown::HashSet;
use zwohash::ZwoHasher;
use common::ProduceKey;

create_benchmark! (hashbrown, zwohasher, ZwoHasher);
