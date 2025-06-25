#![allow(deprecated, reason = "Just benchmarking")]

mod common;

use core::hash::SipHasher;
use std::collections::HashSet;

create_benchmark! (std_hashset, sip_hash, SipHasher);
