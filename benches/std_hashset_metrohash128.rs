mod common;

use std::collections::HashSet;
use metrohash::MetroHash128;

create_benchmark! (std_hashset, metrohash128, MetroHash128);
