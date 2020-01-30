use criterion::{criterion_group, criterion_main, Criterion};
use keccak_hasher::KeccakHasher;
use memory_db::*;
use reference_trie::{RefTrieDBMut, TrieMut};
use trie_db::DBValue;

fn insert_random_kvs() {
    let mut memdb = MemoryDB::<KeccakHasher, HashKey<_>, DBValue>::default();
    let mut root = Default::default();
    let mut trie = RefTrieDBMut::new(&mut memdb, &mut root);

    for _ in 0..256 {
        let k: [u8; 32] = rand::random();
        let v: [u8; 32] = rand::random();
        let _ = trie.insert(&k, &v);
    }
}

fn insert_256(c: &mut Criterion) {
    c.bench_function("substrate_insert_random_kvs 256", |b| {
        b.iter(|| insert_random_kvs())
    });
}

criterion_group!(substrate_trie, insert_256);
criterion_main!(substrate_trie);
