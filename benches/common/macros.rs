/// This macro creates one benchable (by Criterion benchmark harness) function per entry.
///
/// Entry arguments for full individual call:
/// - operation type to measure; either "setup" or "lookup"
/// - name fragment of `HashSet` type
/// - `HashSet` type, including Key type and Hasher specification
/// - Key type
/// - lowercase ident of key type
/// - name fragment of Hasher
/// - Hasher type
/// - Number of entries to process in `HashSet`
/// - Percentage of entries to actually set in `HashSet`
///
#[macro_export]
macro_rules! create_benchmark {
    // Lowest abstraction: creates benchmark functions of type setup_*
    ($(setup $collname:ident,$colltype:ty,$keytype:ty,$keytypename:ident,$hashername:ident,$hasher:ty,$size:expr,$perc:expr),+) => {
        $(paste::paste! (
            pub fn [< bench_setup_ $size _ $perc _ $keytypename _ $collname _ $hashername >] (criterion: &mut criterion::Criterion) -> usize {
                $crate::common::bench_setup::<$colltype, $size, $perc, $keytype, $hasher>(criterion,
                    // This defines name of function and benchmark entry to create
                    stringify!([< setup_  $size _ $perc _ $keytypename _ $collname _ $hashername >]))
            }
        );)+
    };
    // Lowest abstraction: creates benchmark functions of type lookup_*
    ($(lookup $collname:ident,$colltype:ty,$keytype:ty,$keytypename:ident,$hashername:ident,$hasher:ty,$size:expr,$perc:expr);+) => {
        $(paste::paste! (
            pub fn [< bench_lookup_ $size _ $perc _ $keytypename _ $collname _ $hashername >] (criterion: &mut criterion::Criterion) -> usize {
                $crate::common::bench_lookup::<$colltype, $size, $perc, $keytype, $hasher>(criterion,
                    // This defines name of function and benchmark entry to create
                    stringify!([< lookup_ $size _ $perc _ $keytypename _ $collname _ $hashername >]))
            }
        );)+
    };
    // Low abstraction: creates benchmark functions of type setup_* and lookup_*
    ($groupname:ident, $($tt:tt $collname:ident,$colltype:ty,$keytype:ty,$keytypename:ident,$hashername:ident,$hasher:ty,$size:expr,$perc:expr),+) => {
        paste::paste! (
            $(create_benchmark! ($tt $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, $size, $perc);)+
            criterion::criterion_group! (name = $groupname; config = $crate::common::create_criterion();
                targets=$([< bench_ $tt _ $size _ $perc _ $keytypename _ $collname _ $hashername >]),+););
    };
    // High abstraction: creates  15 benchmark functions per collection, key and hasher type
    ($groupname:ident, $($collname:ident,$colltype:ty,$keytype:ty,$keytypename:ident,$hashername:ident,$hasher:ty),+) => {
        $(create_benchmark! (setup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 10, 100);)+
        $(create_benchmark! (setup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 100, 100);)+
        $(create_benchmark! (setup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 1000, 100);)+
        $(create_benchmark! (setup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 10000, 100);)+
        $(create_benchmark! (setup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 100000, 100);)+

        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 10, 100);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 100, 100);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 1000, 100);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 10000, 100);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 100000, 100);)+

        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 10, 10);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 100, 10);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 1000, 10);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 10000, 10);)+
        $(create_benchmark! (lookup $collname, $colltype, $keytype, $keytypename, $hashername, $hasher, 100000, 10);)+

        pub fn $groupname () {
            let mut criterion = $crate::common::create_criterion ();
            paste::paste! ($(
                [< bench_ setup _ 10 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ setup _ 100 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ setup _ 1000 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ setup _ 10000 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ setup _ 100000 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);

                [< bench_ lookup _ 10 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 100 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 1000 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 10000 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 100000 _ 100 _ $keytypename _ $collname _ $hashername >] (&mut criterion);

                [< bench_ lookup _ 10 _ 10 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 100 _ 10 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 1000 _ 10 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 10000 _ 10 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
                [< bench_ lookup _ 100000 _ 10 _ $keytypename _ $collname _ $hashername >] (&mut criterion);
        )+); }
    };
    // Highest abstraction: creates  15*8 benchmark functions per collection and hasher type and instantiates Criterion
    ($groupname:ident,$hashername:ident,$hasher:ty) => {
        criterion::criterion_main! ($groupname);
        $crate::create_benchmark! ($groupname,
            $groupname, HashSet<u32,                            core::hash::BuildHasherDefault<$hasher>>,   u32,                            u32,            $hashername,    $hasher,
            $groupname, HashSet<usize,                          core::hash::BuildHasherDefault<$hasher>>,   usize,                          usize,          $hashername,    $hasher,
            $groupname, HashSet<u128,                           core::hash::BuildHasherDefault<$hasher>>,   u128,                           u128,           $hashername,    $hasher,
            $groupname, HashSet<String,                         core::hash::BuildHasherDefault<$hasher>>,   String,                         string,         $hashername,    $hasher,
            $groupname, HashSet<$crate::common::String8,        core::hash::BuildHasherDefault<$hasher>>,   $crate::common::String8,        string8,        $hashername,    $hasher,
            $groupname, HashSet<$crate::common::String16,       core::hash::BuildHasherDefault<$hasher>>,   $crate::common::String16,       string16,       $hashername,    $hasher,
            $groupname, HashSet<$crate::common::String32,       core::hash::BuildHasherDefault<$hasher>>,   $crate::common::String32,       string32,       $hashername,    $hasher,
            $groupname, HashSet<$crate::common::StringFmtDyn,   core::hash::BuildHasherDefault<$hasher>>,   $crate::common::StringFmtDyn,   stringfmtdyn,   $hashername,    $hasher);
    };
}
