#![allow(deprecated, reason = "Just benchmarking")]

mod common;

use core::hash::SipHasher;
use hashbrown::HashSet;
use common::ProduceKey;

create_benchmark! (hashbrown, sip_hash, SipHasher);
