mod common;

use hashbrown::HashSet;
use ritehash::FxHasher64;

create_benchmark! (hashbrown, fxhasher64, FxHasher64);
