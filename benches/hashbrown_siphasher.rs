#![feature(random)]
#![allow(deprecated, reason = "Just benchmarking")]

mod common;

use core::hash::SipHasher;
use hashbrown::HashSet;

create_benchmark! (hashbrown, sip_hash, SipHasher);
