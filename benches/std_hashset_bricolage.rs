mod common;

use std::collections::HashSet;
use hashers::pigeon::Bricolage;
use common::ProduceKey;

create_benchmark! (std_hashset, bricolage, Bricolage);
