#![allow(
    unused_imports,
    reason = "Exporting some symbols just for bench modules"
)]

mod framework;
mod hashsettrait;
mod keytype;
mod macros;

use core::hash::BuildHasherDefault;
use core::hash::Hasher;
use criterion::Criterion;

pub (crate) use framework::create_criterion;
pub (crate) use framework::bench_lookup;
pub (crate) use framework::bench_setup;
pub (crate) use hashsettrait::HashSetTrait;
pub (crate) use keytype::ProduceKey;
pub (crate) use keytype::String16;
pub (crate) use keytype::String128;
pub (crate) use keytype::String1024;
pub (crate) use keytype::StringFmtDyn;

pub(crate) const WARM_UP_TIME_SECS: f32 = 0.1; // Enough for nearly 99% precision
pub(crate) const MEASUREMENT_TIME_SECS: f32 = 0.2; // Enough for nearly 99% precision
pub(crate) const NOISE_THRESHOLD: f32 = 0.05;
pub(crate) const SAMPLE_SIZE: usize = 10; // Minimum 10, else Criterion panics. Larger numbers increase precision & duration.
