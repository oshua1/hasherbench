mod common;

use std::collections::HashSet;
use metrohash::MetroHash128;
use common::ProduceKey;

create_benchmark! (std_hashset, metrohash128, MetroHash128);
