mod common;

use std::collections::HashSet;
use hashers::jenkins::OAATHasher;

create_benchmark! (std_hashset, oaathasher, OAATHasher);
