#![feature(random)]

mod common;

use std::collections::HashSet;
use highhash::city::CityHasher64;

create_benchmark! (std_hashset, cityhasher64, CityHasher64);
