use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kelvin::{Blake2b, Store};
use kelvin_radix::DefaultRadixMap as Radix;

fn insert_random_kvs() {
    let mut radix = Radix::<_, _, Blake2b>::new();

    for _ in 0..256 {
        let k: [u8; 32] = rand::random();
        let v: [u8; 32] = rand::random();
        let _ = radix.insert(k, v);
    }
}

fn insert_256(c: &mut Criterion) {
    c.bench_function("kelvin_radix_insert_random_kvs 256", |b| {
        b.iter(|| insert_random_kvs())
    });
}

fn insert_persist_random_kvs(store: &Store<Blake2b>) {
    let mut radix = Radix::<[u8; 32], [u8; 32], Blake2b>::new();

    for _ in 0..256 {
        let k: [u8; 32] = rand::random();
        let v: [u8; 32] = rand::random();
        let _ = radix.insert(k, v);
    }

    let _snapshot = store.persist(&mut radix);
}

fn insert_persist_256(c: &mut Criterion) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::new(dir.path()).unwrap();

    c.bench_function("kelvin_radix_insert_persist_random_kvs 256", |b| {
        b.iter(|| insert_persist_random_kvs(black_box(&store)))
    });
}

criterion_main!(kelvin_radix);
criterion_group!(kelvin_radix, insert_256, insert_persist_256);
