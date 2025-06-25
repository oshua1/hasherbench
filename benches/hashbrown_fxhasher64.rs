mod common;

use hashbrown::HashSet;
use ritehash::FxHasher64;
use common::ProduceKey;

create_benchmark! (hashbrown, fxhasher64, FxHasher64);
