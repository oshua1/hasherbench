#![feature(random)]
extern crate alloc;

mod common;

use alloc::collections::BTreeSet;
use common::HashSetTrait;
use common::ProduceKey;
use common::String16;
use common::String128;
use common::String1024;
use common::StringFmtDyn;
use std::hash::DefaultHasher;

criterion::criterion_main!(btreeset);

create_benchmark! (btreeset,
    // Hasher is not used by alloc::collections::BTreeSet
    setup btreeset,     BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  10, 100,
    setup btreeset,     BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  100, 100,
    setup btreeset,     BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  1000, 100,
    setup btreeset,     BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  10000, 100,
    setup btreeset,     BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  100000, 100,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  10, 100,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  100, 100,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  1000, 100,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  10000, 100,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  100000, 100,
    setup btreeset,     BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  10, 100,
    setup btreeset,     BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  100, 100,
    setup btreeset,     BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  1000, 100,
    setup btreeset,     BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  10000, 100,
    setup btreeset,     BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  100000, 100,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  10, 100,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  100, 100,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  1000, 100,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  10000, 100,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  100000, 100,
    setup btreeset,     BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  10, 100,
    setup btreeset,     BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  100, 100,
    setup btreeset,     BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  1000, 100,
    setup btreeset,     BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  10000, 100,
    setup btreeset,     BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  100000, 100,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  10, 100,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  100, 100,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  1000, 100,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  10000, 100,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  100000, 100,
    setup btreeset,     BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  10, 100,
    setup btreeset,     BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  100, 100,
    setup btreeset,     BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  1000, 100,
    setup btreeset,     BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  10000, 100,
    setup btreeset,     BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  100000, 100,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  10, 100,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  100, 100,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  1000, 100,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  10000, 100,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  100000, 100,
    setup btreeset,     BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  10, 100,
    setup btreeset,     BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  100, 100,
    setup btreeset,     BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  1000, 100,
    setup btreeset,     BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  10000, 100,
    setup btreeset,     BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  100000, 100,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  10, 100,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  100, 100,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  1000, 100,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  10000, 100,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  100000, 100,
    setup btreeset,     BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  10, 100,
    setup btreeset,     BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  100, 100,
    setup btreeset,     BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  1000, 100,
    setup btreeset,     BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  10000, 100,
    setup btreeset,     BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  100000, 100,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  10, 100,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  100, 100,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  1000, 100,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  10000, 100,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  100000, 100,
    setup btreeset,     BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  10, 100,
    setup btreeset,     BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  100, 100,
    setup btreeset,     BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  1000, 100,
    setup btreeset,     BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  10000, 100,
    setup btreeset,     BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  100000, 100,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  10, 100,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  100, 100,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  1000, 100,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  10000, 100,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  100000, 100,
    setup btreeset,     BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  10, 100,
    setup btreeset,     BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  100, 100,
    setup btreeset,     BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  1000, 100,
    setup btreeset,     BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  10000, 100,
    setup btreeset,     BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  100000, 100,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  10, 100,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  100, 100,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  1000, 100,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  10000, 100,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  100000, 100,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  10, 10,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  100, 10,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  1000, 10,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  10000, 10,
    lookup btreeset,    BTreeSet<u32>,          u32,            u32,            default_hasher, DefaultHasher,  100000, 10,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  10, 10,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  100, 10,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  1000, 10,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  10000, 10,
    lookup btreeset,    BTreeSet<usize>,        usize,          usize,          default_hasher, DefaultHasher,  100000, 10,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  10, 10,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  100, 10,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  1000, 10,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  10000, 10,
    lookup btreeset,    BTreeSet<u128>,         u128,           u128,           default_hasher, DefaultHasher,  100000, 10,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  10, 10,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  100, 10,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  1000, 10,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  10000, 10,
    lookup btreeset,    BTreeSet<String>,       String,         string,         default_hasher, DefaultHasher,  100000, 10,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  10, 10,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  100, 10,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  1000, 10,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  10000, 10,
    lookup btreeset,    BTreeSet<String16>,     String16,       string16,       default_hasher, DefaultHasher,  100000, 10,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  10, 10,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  100, 10,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  1000, 10,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  10000, 10,
    lookup btreeset,    BTreeSet<String128>,    String128,      string128,      default_hasher, DefaultHasher,  100000, 10,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  10, 10,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  100, 10,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  1000, 10,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  10000, 10,
    lookup btreeset,    BTreeSet<String1024>,   String1024,     string1024,     default_hasher, DefaultHasher,  100000, 10,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  10, 10,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  100, 10,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  1000, 10,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  10000, 10,
    lookup btreeset,    BTreeSet<StringFmtDyn>, StringFmtDyn,   stringfmtdyn,   default_hasher, DefaultHasher,  100000, 10);

impl<KEY: ProduceKey> HashSetTrait<KEY, DefaultHasher> for BTreeSet<KEY> {
    #[inline]
    fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }
    #[inline]
    fn insert(&mut self, key: KEY) {
        let _ = self.insert(key);
    }
    #[inline]
    fn get<'a: 'b, 'b>(&'a self, key: &'b KEY) -> Option<&'b KEY> {
        self.get(key)
    }
    #[inline]
    fn shrink_to_fit(&mut self) -> &mut Self {
        self
    }
    #[inline]
    fn len(&self) -> usize {
        Self::len(self)
    }
}
