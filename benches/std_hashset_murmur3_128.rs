mod common;

use std::collections::HashSet;
use highhash::murmur::Murmur3Hasher128;

create_benchmark! (std_hashset, murmur3_128, Murmur3Hasher128);
