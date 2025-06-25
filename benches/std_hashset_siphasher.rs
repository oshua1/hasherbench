#![allow(deprecated, reason = "Just benchmarking")]

mod common;

use core::hash::SipHasher;
use std::collections::HashSet;
use common::ProduceKey;

create_benchmark! (std_hashset, sip_hash, SipHasher);
