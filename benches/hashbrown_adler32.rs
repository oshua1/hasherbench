mod common;

use adler2::Adler32;
use hashbrown::HashSet;
use common::ProduceKey;

create_benchmark! (hashbrown, adler32, Adler32);
