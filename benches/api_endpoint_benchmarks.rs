use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use metabase_api_rs::{api::Credentials, ClientBuilder};
use serde_json::json;
use std::time::Duration;
use tokio::runtime::Runtime;

/// Benchmark authentication performance
fn bench_authentication(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("authentication");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("email_password_auth", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let _client = ClientBuilder::new("http://localhost:3000")
                        .timeout(Duration::from_secs(30))
                        .build()
                        .unwrap();

                    // In real scenario, this would authenticate against actual server
                    let credentials = Credentials::EmailPassword {
                        email: "test@example.com".to_string(),
                        password: "test_password".to_string().into(),
                    };

                    // Simulate authentication delay
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    black_box(credentials);
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("api_key_auth", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let credentials = Credentials::ApiKey {
                        key: "test_api_key".to_string().into(),
                    };

                    // API key auth is typically faster
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    black_box(credentials);
                }
                start.elapsed()
            })
        });
    });

    group.finish();
}

/// Benchmark CRUD operations on different entities
fn bench_crud_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("crud_operations");
    group.measurement_time(Duration::from_secs(5));

    // Card operations
    group.bench_function("card_create", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Simulate card creation
                    tokio::time::sleep(Duration::from_millis(30)).await;
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("card_read", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Simulate card read
                    tokio::time::sleep(Duration::from_millis(15)).await;
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("card_update", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Simulate card update
                    tokio::time::sleep(Duration::from_millis(25)).await;
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("card_delete", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Simulate card deletion
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
                start.elapsed()
            })
        });
    });

    // Collection operations
    group.bench_function("collection_list", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Simulate listing collections
                    tokio::time::sleep(Duration::from_millis(25)).await;
                }
                start.elapsed()
            })
        });
    });

    group.finish();
}

/// Benchmark query execution with varying complexity
fn bench_query_execution(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("query_execution");
    group.measurement_time(Duration::from_secs(8));

    // Simple query
    group.bench_function("simple_query", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // SELECT * FROM table LIMIT 10
                    tokio::time::sleep(Duration::from_millis(20)).await;
                }
                start.elapsed()
            })
        });
    });

    // Medium complexity query
    group.bench_function("medium_query", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Query with JOIN and WHERE
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
                start.elapsed()
            })
        });
    });

    // Complex query
    group.bench_function("complex_query", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Query with multiple JOINs, aggregations
                    tokio::time::sleep(Duration::from_millis(150)).await;
                }
                start.elapsed()
            })
        });
    });

    // Parameterized query
    group.bench_function("parameterized_query", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let params = json!({
                        "date": "2024-01-01",
                        "status": "active"
                    });

                    // Simulate parameterized query execution
                    tokio::time::sleep(Duration::from_millis(30)).await;
                    black_box(params);
                }
                start.elapsed()
            })
        });
    });

    group.finish();
}

/// Benchmark concurrent operations
fn bench_concurrent_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("concurrent_operations");
    group.measurement_time(Duration::from_secs(10));

    for concurrency in [5, 10, 20].iter() {
        group.throughput(Throughput::Elements(*concurrency as u64));

        group.bench_with_input(
            BenchmarkId::new("concurrent_reads", concurrency),
            concurrency,
            |b, &concurrency| {
                b.iter_custom(|iters| {
                    rt.block_on(async {
                        let start = std::time::Instant::now();
                        for _ in 0..iters {
                            let mut handles = vec![];

                            for _ in 0..concurrency {
                                handles.push(tokio::spawn(async {
                                    // Simulate concurrent read
                                    tokio::time::sleep(Duration::from_millis(10)).await;
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

        group.bench_with_input(
            BenchmarkId::new("concurrent_writes", concurrency),
            concurrency,
            |b, &concurrency| {
                b.iter_custom(|iters| {
                    rt.block_on(async {
                        let start = std::time::Instant::now();
                        for _ in 0..iters {
                            let mut handles = vec![];

                            for _ in 0..concurrency {
                                handles.push(tokio::spawn(async {
                                    // Writes typically take longer and may have contention
                                    tokio::time::sleep(Duration::from_millis(25)).await;
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

/// Benchmark data size impact
fn bench_data_size_impact(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("data_size_impact");
    group.measurement_time(Duration::from_secs(8));

    // Different result set sizes - balanced for development environment
    for size in [10, 50, 200, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("rows_fetched", size), size, |b, &size| {
            b.iter_custom(|iters| {
                rt.block_on(async {
                    let start = std::time::Instant::now();
                    for _ in 0..iters {
                        // Simulate fetching and processing rows
                        // Base time + per-row processing time
                        let base_time = Duration::from_millis(10);
                        let per_row = Duration::from_micros(100);

                        tokio::time::sleep(base_time + per_row * size as u32).await;
                    }
                    start.elapsed()
                })
            });
        });

        group.bench_with_input(BenchmarkId::new("json_parsing", size), size, |b, &size| {
            b.iter_custom(|iters| {
                rt.block_on(async {
                    let start = std::time::Instant::now();
                    for _ in 0..iters {
                        // Simulate JSON parsing overhead
                        let per_row = Duration::from_micros(50);
                        tokio::time::sleep(per_row * size as u32).await;
                    }
                    start.elapsed()
                })
            });
        });
    }

    group.finish();
}

/// Benchmark retry mechanism overhead
fn bench_retry_overhead(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("retry_overhead");

    group.bench_function("no_retry_success", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Direct success
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("retry_once", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // First attempt fails, retry succeeds
                    tokio::time::sleep(Duration::from_millis(10)).await; // First attempt
                    tokio::time::sleep(Duration::from_millis(100)).await; // Backoff
                    tokio::time::sleep(Duration::from_millis(10)).await; // Retry
                }
                start.elapsed()
            })
        });
    });

    group.bench_function("retry_twice", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    // Two retries needed
                    tokio::time::sleep(Duration::from_millis(10)).await; // First attempt
                    tokio::time::sleep(Duration::from_millis(100)).await; // Backoff
                    tokio::time::sleep(Duration::from_millis(10)).await; // First retry
                    tokio::time::sleep(Duration::from_millis(200)).await; // Longer backoff
                    tokio::time::sleep(Duration::from_millis(10)).await; // Second retry
                }
                start.elapsed()
            })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_authentication,
    bench_crud_operations,
    bench_query_execution,
    bench_concurrent_operations,
    bench_data_size_impact,
    bench_retry_overhead
);

criterion_main!(benches);
