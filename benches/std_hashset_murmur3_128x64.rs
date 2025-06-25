mod common;

use std::collections::HashSet;
use highhash::murmur::Murmur3Hasher128x64;

create_benchmark! (std_hashset, murmur3_128x64, Murmur3Hasher128x64);
