mod common;

use std::collections::HashSet;
use metrohash::MetroHash64;
use common::ProduceKey;

create_benchmark! (std_hashset, metrohash64, MetroHash64);
