mod common;

use hashbrown::HashSet;
use wyhash::WyHash;

create_benchmark! (hashbrown, wyhash, WyHash);
