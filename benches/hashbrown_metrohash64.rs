mod common;

use hashbrown::HashSet;
use metrohash::MetroHash64;
use common::ProduceKey;

create_benchmark! (hashbrown, metrohash64, MetroHash64);
