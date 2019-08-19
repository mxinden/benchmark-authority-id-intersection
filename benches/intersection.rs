#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use primitives::{
    crypto::{key_types, Pair},
    sr25519,
    traits::BareCryptoStore,
    H256,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench("intersection", criterion::Benchmark::new("intersection", |b| {
            b.iter_batched_ref(
                || {
                    // The set of authorities.
                    let key_set_a = vec![(); 1000]
                        .iter()
                        .map(|_x| sr25519::Pair::generate_with_phrase(None).0.public())
                        .collect();

                    // The set of authority keys we have in our key store.
                    let key_set_b = vec![(); 10]
                        .iter()
                        .map(|_x| sr25519::Pair::generate_with_phrase(None).0.public())
                        .collect();

                    (key_set_a, key_set_b)
                },
                |sets: &mut (Vec<sr25519::Public>, Vec<sr25519::Public>)| {
                    let (key_set_a, key_set_b) = sets;
                    let result: Option<sr25519::Public> =
                        key_set_a.into_iter().find_map(|authority| {
                            if key_set_b.contains(&authority) {
                                panic!("This should never happen.");
                            } else {
                                None
                            }
                        });

                    // Make sure the compiler does not remove the actual computation
                    // result as it is unused.
                    criterion::black_box(result);
                },
                criterion::BatchSize::SmallInput,
            )
        }).measurement_time(std::time::Duration::from_secs(60)).sample_size(10));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
