mod common;

use hashbrown::HashSet;
use highhash::city::CityHasher64;
use common::ProduceKey;

create_benchmark! (hashbrown, cityhasher64, CityHasher64);
