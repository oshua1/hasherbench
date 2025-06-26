#![feature(random)]

mod common;

use hashbrown::HashSet;
use hashers::pigeon::Bricolage;

create_benchmark! (hashbrown, bricolage, Bricolage);
