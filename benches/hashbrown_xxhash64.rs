mod common;

use hashbrown::HashSet;
use twox_hash::XxHash64;

create_benchmark! (hashbrown, xxhash64, XxHash64);
