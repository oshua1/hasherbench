mod common;

use hashbrown::HashSet;
use highhash::murmur::Murmur3Hasher32;
use common::ProduceKey;

create_benchmark! (hashbrown, murmur3_32, Murmur3Hasher32);
