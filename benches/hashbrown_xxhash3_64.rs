mod common;

use hashbrown::HashSet;
use twox_hash::XxHash3_64;

create_benchmark! (hashbrown, xxhash3_64, XxHash3_64);
