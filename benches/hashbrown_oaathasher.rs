mod common;

use hashbrown::HashSet;
use hashers::jenkins::OAATHasher;

create_benchmark! (hashbrown, oaathasher, OAATHasher);
