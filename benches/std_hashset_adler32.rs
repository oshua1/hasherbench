mod common;

use std::collections::HashSet;
use adler2::Adler32;

create_benchmark! (std_hashset, adler32, Adler32);
