use core::hash::Hasher;

use criterion::Criterion;

use crate::common::HashSetTrait;
use crate::ProduceKey;

pub (crate) fn create_criterion() -> Criterion {
    criterion::Criterion::default()
                .warm_up_time(core::time::Duration::from_secs_f32(crate::common::WARM_UP_TIME_SECS))
                .measurement_time(core::time::Duration::from_secs_f32(crate::common::MEASUREMENT_TIME_SECS))
                .noise_threshold(f64::from (crate::common::NOISE_THRESHOLD))
                .sample_size(crate::common::SAMPLE_SIZE)
                .confidence_level(0.9)
                .configure_from_args()
}

#[must_use]
fn setup_hashset<
    T: HashSetTrait<K, H>,
    const SIZE: usize,
    const PERC: u8,
    K: ProduceKey,
    H: Hasher,
>() -> T {
    let mut hs = T::with_capacity(SIZE);
    (0..SIZE)
        .filter(|idx| idx % 100 < usize::from(PERC))
        .for_each(|idx| {
            hs.insert(K::produce_key(idx));
        });
    hs.shrink_to_fit();
    hs
}

pub(crate) fn bench_setup<
    T: HashSetTrait<K, H>,
    const SIZE: usize,
    const PERC: u8,
    K: ProduceKey,
    H: Hasher,
>(
    criterion: &mut Criterion,
    funcname: &str,
) -> usize {
    let mut count: usize = 0;
    criterion.bench_function(funcname, |b| {
        b.iter(|| count += setup_hashset::<T, SIZE, PERC, K, H>().len());
    });
    count // Return some value to avoid optimizing iterations away
}

pub(crate) fn bench_lookup<
    T: HashSetTrait<K, H>,
    const SIZE: usize,
    const PERC: u8,
    K: ProduceKey,
    H: Hasher,
>(
    criterion: &mut Criterion,
    funcname: &str,
) -> usize {
    let hs = setup_hashset::<T, SIZE, PERC, K, H>();
    let mut result = 0;
    criterion.bench_function(funcname, |b| {
        b.iter(|| {
            (0..SIZE).for_each(|idx| {
                hs.get(&K::produce_key(idx)).and_then(|_| {
                    result += 1;
                    None::<usize>
                });
            });
        });
    });
    result // Return some value to avoid optimizing iterations away
}
