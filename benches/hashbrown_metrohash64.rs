mod common;

use hashbrown::HashSet;
use metrohash::MetroHash64;

create_benchmark! (hashbrown, metrohash64, MetroHash64);
