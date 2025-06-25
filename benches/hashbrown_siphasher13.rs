#![feature (hashmap_internals)]
#![allow(internal_features, deprecated, reason = "Just benchmarking")]

mod common;

use core::hash::SipHasher13;
use hashbrown::HashSet;
use common::ProduceKey;

create_benchmark! (hashbrown, sip_hash13, SipHasher13);
