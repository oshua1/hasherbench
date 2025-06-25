mod common;

use hashbrown::HashSet;
use wyhash::WyHash;
use common::ProduceKey;

create_benchmark! (hashbrown, wyhash, WyHash);
