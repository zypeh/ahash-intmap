use ahash::{AHashMap, AHasher};
use criterion::Throughput;
use criterion::{black_box, BenchmarkId};
use criterion::{criterion_group, criterion_main, Criterion};
use intmap::IntMap;
use std::collections::HashMap;
use std::hash::Hasher;

use rand::{
    distributions::{DistString, Standard},
    Rng,
};

fn get_random_tick(count: usize) -> Vec<String> {
    let mut vec = Vec::with_capacity(count);

    for _ in 0..count {
        let len = rand::thread_rng().gen_range(6..9); // [6..9]
        vec.push(Standard.sample_string(&mut rand::thread_rng(), len));
    }
    vec.dedup();
    vec
}

fn lookup_ahashintmap(c: &mut Criterion) {
    let DATA_SIZE = 10000;
    let data = get_random_tick(DATA_SIZE);

    let mut map = IntMap::with_capacity(DATA_SIZE);
    for datum in data.clone() {
        let mut ahasher = AHasher::default();
        ahasher.write(datum.as_bytes());
        black_box(map.insert(ahasher.finish(), datum));
    }

    c.bench_function("lookup ahash-intmap", |b| {
        b.iter(|| {
            // Code to benchmark goes here
            for i in data.clone() {
                let mut ahasher = AHasher::default();
                ahasher.write(i.as_bytes());
                black_box(map.get(ahasher.finish()));
            }
        })
    });
}

fn lookup_hashmap(c: &mut Criterion) {
    let DATA_SIZE = 10000;
    let data = get_random_tick(DATA_SIZE);

    let mut map = HashMap::with_capacity(DATA_SIZE);
    for datum in data.clone() {
        black_box(map.insert(datum.clone(), datum));
    }

    c.bench_function("lookup hashmap", |b| {
        b.iter(|| {
            // Code to benchmark goes here
            for i in data.clone() {
                black_box(map.get(&i));
            }
        })
    });
}

fn lookup_ahashmap(c: &mut Criterion) {
    let DATA_SIZE = 10000;
    let data = get_random_tick(DATA_SIZE);

    let mut map = AHashMap::with_capacity(DATA_SIZE);
    for datum in data.clone() {
        black_box(map.insert(datum.clone(), datum));
    }

    c.bench_function("lookup ahashmap", |b| {
        b.iter(|| {
            // Code to benchmark goes here
            for i in data.clone() {
                black_box(map.get(&i));
            }
        })
    });
}

criterion_group!(benches, lookup_ahashintmap, lookup_hashmap, lookup_ahashmap);
criterion_main!(benches);
