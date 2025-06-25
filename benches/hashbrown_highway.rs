mod common;

use hashbrown::HashSet;
use highway::HighwayHasher;
use common::ProduceKey;

create_benchmark! (hashbrown, highwayhasher, HighwayHasher);
