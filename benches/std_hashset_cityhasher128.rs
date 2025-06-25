mod common;

use std::collections::HashSet;
use highhash::city::CityHasher128;

create_benchmark! (std_hashset, cityhasher128, CityHasher128);
