mod common;

use hashbrown::HashSet;
use hashers::jenkins::Lookup3Hasher;
use common::ProduceKey;

create_benchmark! (hashbrown, lookup3hasher, Lookup3Hasher);
