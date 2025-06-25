mod common;

use hashbrown::HashSet;
use hashers::oz::DJB2Hasher;

create_benchmark! (hashbrown, djb2hasher, DJB2Hasher);
