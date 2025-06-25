mod common;

use hashbrown::HashSet;
use highhash::murmur::Murmur3Hasher32;

create_benchmark! (hashbrown, murmur3_32, Murmur3Hasher32);
