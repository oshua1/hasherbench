/// This macro creates one benchable (by Criterion benchmark harness) function per entry.
///
/// Entry arguments for full individual call:
/// - operation type to measure; either "setup" or "lookup"
/// - name fragment of `HashSet` type
/// - `HashSet` type, including Key type and Hasher specification
/// - Key type
/// - name fragment of Hasher
/// - Hasher type
/// - Number of entries to process in `HashSet`
/// - Percentage of entries to actually set in `HashSet`
///
#[macro_export]
macro_rules! create_benchmark {
    ($(setup $collname:ident,$colltype:ty,$keytype:ty,$hashername:ident,$hasher:ty,$size:expr,$perc:expr),+) => {
        $(paste::paste! (
            pub fn [< bench_setup_ $size _ $perc _ $keytype _ $collname _ $hashername >] (criterion: &mut criterion::Criterion) -> usize {
                $crate::common::bench_setup::<$colltype, $size, $perc, $keytype, $hasher>(criterion,
                    // This defines name of function and benchmark entry to create
                    stringify!([< setup_  $size _ $perc _ $keytype _ $collname _ $hashername >]))
            }
        );)+
    };
    ($(lookup $collname:ident,$colltype:ty,$keytype:ty,$hashername:ident,$hasher:ty,$size:expr,$perc:expr);+) => {
        $(paste::paste! (
            pub fn [< bench_lookup_ $size _ $perc _ $keytype _ $collname _ $hashername >] (criterion: &mut criterion::Criterion) -> usize {
                $crate::common::bench_lookup::<$colltype, $size, $perc, $keytype, $hasher>(criterion,
                    // This defines name of function and benchmark entry to create
                    stringify!([< lookup_ $size _ $perc _ $keytype _ $collname _ $hashername >]))
            }
        );)+
    };
    ($groupname:ident, $($tt:tt $collname:ident,$colltype:ty,$keytype:ty,$hashername:ident,$hasher:ty,$size:expr,$perc:expr),+) => {
        paste::paste! (
            $(create_benchmark! ($tt $collname, $colltype, $keytype, $hashername, $hasher, $size, $perc);)+
            criterion::criterion_group! (name = $groupname; config = $crate::common::create_criterion();
                targets=$([< bench_ $tt _ $size _ $perc _ $keytype _ $collname _ $hashername >]),+););
    };
    ($groupname:ident, $($collname:ident,$colltype:ty,$keytype:ty,$hashername:ident,$hasher:ty),+) => {
        $(create_benchmark! (setup $collname, $colltype, $keytype, $hashername, $hasher, 10, 100);)+
        $(create_benchmark! (setup $collname, $colltype, $keytype, $hashername, $hasher, 100, 100);)+
        $(create_benchmark! (setup $collname, $colltype, $keytype, $hashername, $hasher, 1000, 100);)+
        $(create_benchmark! (setup $collname, $colltype, $keytype, $hashername, $hasher, 10000, 100);)+
        $(create_benchmark! (setup $collname, $colltype, $keytype, $hashername, $hasher, 100000, 100);)+

        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 10, 100);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 100, 100);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 1000, 100);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 10000, 100);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 100000, 100);)+

        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 10, 10);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 100, 10);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 1000, 10);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 10000, 10);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $hashername, $hasher, 100000, 10);)+

        pub fn $groupname () {
            let mut criterion = $crate::common::create_criterion ();
            paste::paste! ($(
                [< bench_ setup _ 10 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ setup _ 100 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ setup _ 1000 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ setup _ 10000 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ setup _ 100000 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);

                [< bench_ lookup _ 10 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 100 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 1000 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 10000 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 100000 _ 100 _ $keytype _ $collname _ $hashername >] (&mut criterion);

                [< bench_ lookup _ 10 _ 10 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 100 _ 10 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 1000 _ 10 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 10000 _ 10 _ $keytype _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 100000 _ 10 _ $keytype _ $collname _ $hashername >] (&mut criterion);
        )+); }
    };
}
