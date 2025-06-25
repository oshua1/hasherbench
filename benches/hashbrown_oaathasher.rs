mod common;

use hashbrown::HashSet;
use hashers::jenkins::OAATHasher;
use common::ProduceKey;

create_benchmark! (hashbrown, oaathasher, OAATHasher);
