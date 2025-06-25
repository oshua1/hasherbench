mod common;

use hashbrown::HashSet;
use metrohash::MetroHash128;
use common::ProduceKey;

create_benchmark! (hashbrown, metrohash128, MetroHash128);
