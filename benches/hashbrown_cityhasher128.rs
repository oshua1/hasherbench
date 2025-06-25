mod common;

use hashbrown::HashSet;
use highhash::city::CityHasher128;
use common::ProduceKey;

create_benchmark! (hashbrown, cityhasher128, CityHasher128);
