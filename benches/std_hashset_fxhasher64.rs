mod common;

use std::collections::HashSet;
use ritehash::FxHasher64;

create_benchmark! (std_hashset, fxhasher64, FxHasher64);
