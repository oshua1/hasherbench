#![feature(random)]

mod common;

use hashbrown::HashSet;
use highhash::city::CityHasher64;

create_benchmark! (hashbrown, cityhasher64, CityHasher64);
