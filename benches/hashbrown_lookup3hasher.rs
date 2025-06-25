mod common;

use hashbrown::HashSet;
use hashers::jenkins::Lookup3Hasher;

create_benchmark! (hashbrown, lookup3hasher, Lookup3Hasher);
