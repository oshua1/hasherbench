mod common;

use hashers::jenkins::spooky_hash::SpookyHasher;
use hashbrown::HashSet;

create_benchmark! (hashbrown, spookyhasher, SpookyHasher);
