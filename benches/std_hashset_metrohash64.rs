mod common;

use std::collections::HashSet;
use metrohash::MetroHash64;

create_benchmark! (std_hashset, metrohash64, MetroHash64);
