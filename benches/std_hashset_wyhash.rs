mod common;

use std::collections::HashSet;
use wyhash::WyHash;

create_benchmark! (std_hashset, wyhash, WyHash);
