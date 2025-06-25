mod common;

use std::collections::HashSet;
use wyhash::WyHash;
use common::ProduceKey;

create_benchmark! (std_hashset, wyhash, WyHash);
