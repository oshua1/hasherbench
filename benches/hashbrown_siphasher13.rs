#![feature(hashmap_internals, random)]
#![allow(internal_features, deprecated, reason = "Just benchmarking")]

mod common;

use core::hash::SipHasher13;
use hashbrown::HashSet;

create_benchmark! (hashbrown, sip_hash13, SipHasher13);
