mod common;

use std::collections::HashSet;
use highhash::city::CityHasher32;
use common::ProduceKey;

create_benchmark! (std_hashset, cityhasher32, CityHasher32);
