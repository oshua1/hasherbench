mod common;

use std::collections::HashSet;
use highhash::city::CityHasher64;
use common::ProduceKey;

create_benchmark! (std_hashset, cityhasher64, CityHasher64);
