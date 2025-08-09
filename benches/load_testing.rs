use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use tokio::sync::Semaphore;

/// Simulate a load test scenario with varying number of concurrent users
fn bench_load_test_scenarios(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("load_testing");
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10); // Reduce sample size for load tests

    // Test with different numbers of concurrent users
    for num_users in [5, 10, 20].iter() {
        group.throughput(Throughput::Elements(*num_users as u64));

        group.bench_with_input(
            BenchmarkId::new("concurrent_users", num_users),
            num_users,
            |b, &num_users| {
                b.iter_custom(|iters| {
                    rt.block_on(async {
                        let start = std::time::Instant::now();
                        for _ in 0..iters {
                            simulate_user_load(num_users).await;
                        }
                        start.elapsed()
                    })
                });
            },
        );
    }

    group.finish();
}

/// Simulate realistic user behavior patterns
async fn simulate_user_load(num_users: usize) {
    let semaphore = Arc::new(Semaphore::new(num_users));
    let mut handles = vec![];

    for user_id in 0..num_users {
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let handle = tokio::spawn(async move {
            // Simulate a user session
            simulate_user_session(user_id).await;
            drop(permit);
        });

        handles.push(handle);
    }

    // Wait for all users to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

/// Simulate a single user session with typical actions
async fn simulate_user_session(user_id: usize) {
    // Login
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Browse dashboards
    for _ in 0..3 {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    // Run a query
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Modify a card
    if user_id % 5 == 0 {
        tokio::time::sleep(Duration::from_millis(30)).await;
    }

    // View results
    tokio::time::sleep(Duration::from_millis(15)).await;
}

/// Benchmark sustained load over time (endurance test)
fn bench_sustained_load(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("sustained_load");
    group.measurement_time(Duration::from_secs(60)); // 1 minute sustained load
    group.sample_size(10); // Minimum samples for long-running tests

    group.bench_function("steady_50_users", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let duration = Duration::from_secs(5); // Shorter for benchmark
                    let loop_start = Instant::now();

                    while loop_start.elapsed() < duration {
                        // Maintain steady load of 50 users
                        let handles: Vec<_> = (0..50)
                            .map(|_| {
                                tokio::spawn(async {
                                    // Simulate API call
                                    tokio::time::sleep(Duration::from_millis(10)).await;
                                })
                            })
                            .collect();

                        for handle in handles {
                            handle.await.unwrap();
                        }

                        // Small delay between waves
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
                start.elapsed()
            })
        });
    });

    group.finish();
}

/// Benchmark spike load handling
fn bench_spike_load(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("spike_load");
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("traffic_spike", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Normal load (10 users)
                    simulate_user_load(10).await;

                    // Sudden spike (100 users)
                    simulate_user_load(100).await;

                    // Return to normal
                    simulate_user_load(10).await;
                }
                start.elapsed()
            })
        });
    });

    group.finish();
}

/// Benchmark rate limiting behavior
fn bench_rate_limiting(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("rate_limiting");

    for requests_per_second in [10, 50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::new("rps", requests_per_second),
            requests_per_second,
            |b, &rps| {
                b.iter_custom(|iters| {
                    rt.block_on(async {
                        let start = std::time::Instant::now();
                        for _ in 0..iters {
                            let delay = Duration::from_millis(1000 / rps as u64);
                            let mut handles = vec![];

                            for _ in 0..rps {
                                handles.push(tokio::spawn(async move {
                                    // Simulate API request
                                    tokio::time::sleep(Duration::from_millis(5)).await;
                                }));

                                // Spread requests evenly
                                tokio::time::sleep(delay).await;
                            }

                            for handle in handles {
                                handle.await.unwrap();
                            }
                        }
                        start.elapsed()
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark connection pool saturation
fn bench_connection_pool_saturation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("connection_pool");
    group.measurement_time(Duration::from_secs(15));

    // Assume pool size of 20
    const POOL_SIZE: usize = 20;

    for num_requests in [10, 20, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("pool_saturation", num_requests),
            num_requests,
            |b, &num_requests| {
                b.iter_custom(|iters| {
                    rt.block_on(async {
                        let start = std::time::Instant::now();
                        for _ in 0..iters {
                            let semaphore = Arc::new(Semaphore::new(POOL_SIZE));
                            let mut handles = vec![];

                            for _ in 0..num_requests {
                                let permit = semaphore.clone().acquire_owned().await.unwrap();

                                handles.push(tokio::spawn(async move {
                                    // Simulate connection usage
                                    tokio::time::sleep(Duration::from_millis(10)).await;
                                    drop(permit);
                                }));
                            }

                            for handle in handles {
                                handle.await.unwrap();
                            }
                        }
                        start.elapsed()
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark error recovery under load
fn bench_error_recovery(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("error_recovery");

    group.bench_function("with_10_percent_errors", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let mut handles = vec![];

                    for i in 0..100 {
                        handles.push(tokio::spawn(async move {
                            if i % 10 == 0 {
                                // Simulate error and retry
                                tokio::time::sleep(Duration::from_millis(10)).await; // First attempt
                                tokio::time::sleep(Duration::from_millis(100)).await; // Backoff
                                tokio::time::sleep(Duration::from_millis(10)).await;
                            // Retry
                            } else {
                                // Normal request
                                tokio::time::sleep(Duration::from_millis(10)).await;
                            }
                        }));
                    }

                    for handle in handles {
                        handle.await.unwrap();
                    }
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("with_50_percent_errors", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let mut handles = vec![];

                    for i in 0..100 {
                        handles.push(tokio::spawn(async move {
                            if i % 2 == 0 {
                                // Simulate error and retry
                                tokio::time::sleep(Duration::from_millis(10)).await; // First attempt
                                tokio::time::sleep(Duration::from_millis(100)).await; // Backoff
                                tokio::time::sleep(Duration::from_millis(10)).await;
                            // Retry
                            } else {
                                // Normal request
                                tokio::time::sleep(Duration::from_millis(10)).await;
                            }
                        }));
                    }

                    for handle in handles {
                        handle.await.unwrap();
                    }
                }
                start.elapsed()
            })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_load_test_scenarios,
    bench_sustained_load,
    bench_spike_load,
    bench_rate_limiting,
    bench_connection_pool_saturation,
    bench_error_recovery
);

criterion_main!(benches);
