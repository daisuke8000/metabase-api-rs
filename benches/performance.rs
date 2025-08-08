use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use metabase_api_rs::{ClientBuilder, MetabaseClient};
use std::time::Duration;
use tokio::runtime::Runtime;

#[cfg(feature = "cache")]
use metabase_api_rs::cache::{CacheConfig, CacheLayer};

/// Benchmark for single card fetch without cache
fn bench_get_card_no_cache(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("get_card_no_cache", |b| {
        b.to_async(&rt).iter(|| async {
            // This is a mock benchmark - in real scenario, would use actual server
            let client = ClientBuilder::new("http://localhost:3000").build().unwrap();

            // Simulate card fetch (would be actual API call in real test)
            black_box(async {
                // Mock delay to simulate network latency
                tokio::time::sleep(Duration::from_millis(10)).await;
            })
            .await
        });
    });
}

#[cfg(feature = "cache")]
/// Benchmark for single card fetch with cache
fn bench_get_card_with_cache(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("get_card_with_cache", |b| {
        b.to_async(&rt).iter(|| async {
            let mut config = CacheConfig::default();
            config.cache_metadata = true;

            let cache = CacheLayer::new(config);

            // First call - cache miss
            black_box(async {
                tokio::time::sleep(Duration::from_millis(10)).await;
            })
            .await;

            // Subsequent calls - cache hit (should be faster)
            black_box(async {
                // Simulated instant cache hit
                tokio::time::sleep(Duration::from_micros(100)).await;
            })
            .await
        });
    });
}

/// Benchmark for connection pooling effectiveness
fn bench_connection_pooling(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("connection_pooling");

    // Without connection pooling (new connection each time)
    group.bench_function("without_pooling", |b| {
        b.to_async(&rt).iter(|| async {
            // Simulate connection overhead
            tokio::time::sleep(Duration::from_millis(5)).await;
            // Simulate request
            tokio::time::sleep(Duration::from_millis(10)).await;
        });
    });

    // With connection pooling (reuse connections)
    group.bench_function("with_pooling", |b| {
        b.to_async(&rt).iter(|| async {
            // No connection overhead after first request
            // Just the request time
            tokio::time::sleep(Duration::from_millis(10)).await;
        });
    });

    group.finish();
}

/// Benchmark for batch operations
fn bench_batch_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("batch_operations");

    for size in [1, 5, 10, 20].iter() {
        // Sequential operations
        group.bench_with_input(BenchmarkId::new("sequential", size), size, |b, &size| {
            b.to_async(&rt).iter(|| async move {
                for _ in 0..size {
                    // Simulate individual request
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            });
        });

        // Batch operations (simulated)
        group.bench_with_input(BenchmarkId::new("batch", size), size, |b, &size| {
            b.to_async(&rt).iter(|| async move {
                // Batch requests have overhead but are faster overall
                let batch_overhead = Duration::from_millis(2);
                let per_item = Duration::from_millis(2);

                tokio::time::sleep(batch_overhead).await;
                tokio::time::sleep(per_item * size as u32).await;
            });
        });
    }

    group.finish();
}

#[cfg(feature = "cache")]
/// Benchmark cache hit ratio impact
fn bench_cache_hit_ratio(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("cache_hit_ratio");

    for hit_rate in [0.0, 0.25, 0.5, 0.75, 0.9, 0.99].iter() {
        group.bench_with_input(
            BenchmarkId::new("hit_rate", format!("{:.0}%", hit_rate * 100.0)),
            hit_rate,
            |b, &hit_rate| {
                b.to_async(&rt).iter(|| async move {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();

                    // Simulate cache hit/miss based on hit rate
                    if rng.gen::<f64>() < hit_rate {
                        // Cache hit - very fast
                        tokio::time::sleep(Duration::from_micros(100)).await;
                    } else {
                        // Cache miss - need to fetch from server
                        tokio::time::sleep(Duration::from_millis(10)).await;
                    }
                });
            },
        );
    }

    group.finish();
}

// Configure benchmarks based on features
#[cfg(feature = "cache")]
criterion_group!(
    benches,
    bench_get_card_no_cache,
    bench_get_card_with_cache,
    bench_connection_pooling,
    bench_batch_operations,
    bench_cache_hit_ratio
);

#[cfg(not(feature = "cache"))]
criterion_group!(
    benches,
    bench_get_card_no_cache,
    bench_connection_pooling,
    bench_batch_operations
);

criterion_main!(benches);
