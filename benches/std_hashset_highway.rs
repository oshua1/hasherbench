mod common;

use std::collections::HashSet;
use highway::HighwayHasher;
use common::ProduceKey;

create_benchmark! (std_hashset, highwayhasher, HighwayHasher);
