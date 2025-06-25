mod common;

use std::collections::HashSet;
use highhash::murmur::Murmur3Hasher128x64;
use common::ProduceKey;

create_benchmark! (std_hashset, murmur3_128x64, Murmur3Hasher128x64);
