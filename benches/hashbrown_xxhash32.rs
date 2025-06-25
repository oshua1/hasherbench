mod common;

use hashbrown::HashSet;
use twox_hash::XxHash32;
use common::ProduceKey;

create_benchmark! (hashbrown, xxhash32, XxHash32);
