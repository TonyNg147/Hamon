// A simple repetitive decorator for scaling
struct BufferStep {
    val: u32,
}

impl Decorator<String, String> for BufferStep {
    fn produce(&mut self, _previous: String) -> hamon::errors::Result<String> {
        Ok(format!("{:08x}", self.val))
    }
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hamon::{prelude::Builder, Collector, Decorator};

fn bench_heavy_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hamon Stress Test (50 Steps)");
    let depth = 50;

    // --- SAMPLE 1: THE MONOLITH ---
    // Hard-coded manual loop. No abstraction, just pure speed.
    group.bench_function("Monolith_Manual_Loop", |b| {
        b.iter(|| {
            let mut result = String::new();
            for i in 0..black_box(depth) {
                result = format!("{:08x}", i as u32);
            }
            black_box(result)
        })
    });

    // --- SAMPLE 2: HAMON (The Samurai) ---
    // Recursive static types. Modular, but compiled into a flat machine.
    group.bench_function("Hamon_Static_Lineage", |b| {
        b.iter(|| {
            // We simulate a 50-step chain.
            // In a real app, this would be 50 .step() calls.
            let builder = Builder::new(String::new());

            // To simulate 50 .step() calls without 50 lines of code:
            // (In real Hamon usage, the type grows with every .step())
            macro_rules! add_ten {
                ($b:expr, $offset:expr) => {
                    $b.step(BufferStep {
                        val: ($offset + 0) as u32,
                    })
                    .step(BufferStep {
                        val: ($offset + 1) as u32,
                    })
                    .step(BufferStep {
                        val: ($offset + 2) as u32,
                    })
                    .step(BufferStep {
                        val: ($offset + 3) as u32,
                    })
                    .step(BufferStep {
                        val: ($offset + 4) as u32,
                    })
                    .step(BufferStep {
                        val: ($offset + 5) as u32,
                    })
                    .step(BufferStep {
                        val: ($offset + 6) as u32,
                    })
                    .step(BufferStep {
                        val: ($offset + 7) as u32,
                    })
                    .step(BufferStep {
                        val: ($offset + 8) as u32,
                    })
                    .step(BufferStep {
                        val: ($offset + 9) as u32,
                    })
                };
            }

            let final_builder = add_ten!(
                add_ten!(add_ten!(add_ten!(add_ten!(builder, 0), 10), 20), 30),
                40
            );
            black_box(final_builder.collect().unwrap())
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

            let mut result = String::new();
            for item in items.iter_mut() {
                result = item.produce(result).unwrap();
            }
            black_box(result)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_heavy_usage);
criterion_main!(benches);
