mod common;

use std::collections::HashSet;
use highhash::city::CityHasher32;

create_benchmark! (std_hashset, cityhasher32, CityHasher32);
