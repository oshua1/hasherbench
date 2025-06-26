#![feature(random)]

mod common;

use adler2::Adler32;
use hashbrown::HashSet;

create_benchmark! (hashbrown, adler32, Adler32);
