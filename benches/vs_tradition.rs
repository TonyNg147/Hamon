// A simple repetitive decorator for scaling
struct BufferStep {
    val: u32,
}

impl Decorator<String, String> for BufferStep {
    fn produce(&mut self, _previous: String) -> String {
        format!("{:08x}", self.val)
    }
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hamon::{prelude::Builder, Decorator};
// Import your Hamon components here

fn bench_heavy_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hamon Stress Test (50 Steps)");
    let depth = 50;

    // --- SAMPLE 1: THE MONOLITH ---
    // Hard-coded manual loop. No abstraction, just pure speed.
    group.bench_function("Monolith_Manual_Loop", |b| {
        b.iter(|| {
            let mut results = Vec::with_capacity(black_box(depth));
            for i in 0..depth {
                results.push(format!("{:08x}", i as u32));
            }
            black_box(results)
        })
    });

    // --- SAMPLE 2: HAMON (The Samurai) ---
    // Recursive static types. Modular, but compiled into a flat machine.
    group.bench_function("Hamon_Static_Lineage", |b| {
        b.iter(|| {
            // We simulate a 50-step chain.
            // In a real app, this would be 50 .add() calls.
            let builder = Builder::new(String::new());

            // To simulate 50 .add() calls without 50 lines of code:
            // (In real Hamon usage, the type grows with every .add())
            macro_rules! add_ten {
                ($b:expr) => {
                    $b.step(BufferStep { val: 1 })
                        .step(BufferStep { val: 2 })
                        .step(BufferStep { val: 3 })
                        .step(BufferStep { val: 4 })
                        .step(BufferStep { val: 5 })
                        .step(BufferStep { val: 6 })
                        .step(BufferStep { val: 7 })
                        .step(BufferStep { val: 8 })
                        .step(BufferStep { val: 9 })
                        .step(BufferStep { val: 10 })
                };
            }

            let final_builder = add_ten!(add_ten!(add_ten!(add_ten!(add_ten!(builder)))));
            black_box(final_builder.build())
        })
    });

    // --- SAMPLE 3: TRADITIONAL DYNAMIC ---
    // The "Ronin" way. Flexible at runtime, but heavy.
    group.bench_function("Traditional_Dynamic_Vec", |b| {
        b.iter(|| {
            let mut items: Vec<Box<dyn Decorator<String, String>>> = Vec::with_capacity(depth);
            for i in 0..depth {
                items.push(Box::new(BufferStep { val: i as u32 }));
            }

            let results: Vec<String> = items
                .iter_mut()
                .map(|item| item.produce("test".into()))
                .collect();
            black_box(results)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_heavy_usage);
criterion_main!(benches);
