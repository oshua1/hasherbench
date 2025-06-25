#![feature (hashmap_internals)]
#![allow(internal_features, deprecated, reason = "Just benchmarking")]

mod common;

use core::hash::SipHasher13;
use std::collections::HashSet;
use common::ProduceKey;

create_benchmark! (std_hashset, sip_hash13, SipHasher13);
