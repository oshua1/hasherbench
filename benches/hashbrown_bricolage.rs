mod common;

use hashbrown::HashSet;
use hashers::pigeon::Bricolage;
use common::ProduceKey;

create_benchmark! (hashbrown, bricolage, Bricolage);
