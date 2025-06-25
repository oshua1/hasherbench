mod common;

use hashbrown::HashSet;
use highhash::city::CityHasher32;

create_benchmark! (hashbrown, cityhasher32, CityHasher32);
