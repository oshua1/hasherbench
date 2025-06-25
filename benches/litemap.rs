mod common;

use std::hash::DefaultHasher;
use common::HashSetTrait;
use common::ProduceKey;
use common::String16;
use common::StringSlow;

criterion::criterion_main!(litemap);

create_benchmark! (litemap,
    // Hasher is not used by litemap::LiteMap
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 10, 100,
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 100, 100,
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<u32,()>,                       u32,        u32,        default_hasher, DefaultHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 10, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 100, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<usize,()>,                     usize,      usize,      default_hasher, DefaultHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 10, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 100, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<u128,()>,                      u128,       u128,       default_hasher, DefaultHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 10, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 100, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<String,()>,                    String,     string,     default_hasher, DefaultHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 10, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 100, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<String16,()>,                  String16,   string16,   default_hasher, DefaultHasher, 100000, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 10, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 100, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 1000, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 10000, 100,
    setup litemap,          litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 100000, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 10, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 100, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 1000, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 10000, 100,
    lookup litemap,         litemap::LiteMap<StringSlow,()>,                StringSlow, stringslow, default_hasher, DefaultHasher, 100000, 100);


impl<KEY: ProduceKey> HashSetTrait<KEY, DefaultHasher> for litemap::LiteMap<KEY, ()> {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
    #[inline]
    fn insert(&mut self, key: KEY) {
        let _ = self.insert(key, ());
    }
    #[inline]
    fn get<'a: 'b, 'b>(&'a self, key: &'b KEY) -> Option<&'b KEY> {
        self.get(key).map(|()| key)
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
