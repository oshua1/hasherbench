mod common;

use hashbrown::HashSet;
use highhash::city::CityHasher32;
use common::ProduceKey;

create_benchmark! (hashbrown, cityhasher32, CityHasher32);
