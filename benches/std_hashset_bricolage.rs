mod common;

use std::collections::HashSet;
use hashers::pigeon::Bricolage;

create_benchmark! (std_hashset, bricolage, Bricolage);
