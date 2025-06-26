#![feature(random)]

mod common;

use std::collections::HashSet;
use hashers::oz::DJB2Hasher;

create_benchmark! (std_hashset, djb2hasher, DJB2Hasher);
