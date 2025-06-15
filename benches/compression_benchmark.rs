/// Performance benchmarks for compression system
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use cursed::stdlib::web_vibez::compression::*;
use std::time::Duration;

fn create_test_data(size: usize, pattern: &str) -> Vec<u8> {
    match pattern {
        "repetitive" => vec![b'a'; size],
        "random" => (0..size).map(|i| (i % 256) as u8).collect(),
        "text" => "The quick brown fox jumps over the lazy dog. ".repeat(size / 45 + 1).into_bytes().into_iter().take(size).collect(),
        "html" => format!(r#"<!DOCTYPE html>
<html><head><title>Test Page</title></head>
<body><h1>Header</h1><p>This is paragraph {}.</p></body></html>"#, "content").repeat(size / 100 + 1).into_bytes().into_iter().take(size).collect(),
        _ => vec![0u8; size],
    }
}

fn bench_compression_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_algorithms");
    
    let sizes = vec![1_024, 10_240, 102_400, 1_048_576]; // 1KB, 10KB, 100KB, 1MB
    let patterns = vec!["repetitive", "random", "text", "html"];
    let algorithms = vec![
        CompressionType::Gzip,
        CompressionType::Deflate,
        CompressionType::Brotli,
        CompressionType::Zstd,
    ];
    
    for size in &sizes {
        for pattern in &patterns {
            for algorithm in &algorithms {
                let test_data = create_test_data(*size, pattern);
                group.throughput(Throughput::Bytes(*size as u64));
                
                group.bench_with_input(
                    BenchmarkId::new(
                        format!("{:?}_{}", algorithm, pattern),
                        size
                    ),
                    &test_data,
                    |b, data| {
                        let mut compressor = ResponseCompressor::new();
                        b.iter(|| {
                            black_box(compressor.compress(data, algorithm.clone()).unwrap())
                        })
                    },
                );
            }
        }
    }
    
    group.finish();
}

fn bench_decompression_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("decompression_algorithms");
    
    let sizes = vec![10_240, 102_400, 1_048_576]; // 10KB, 100KB, 1MB
    let algorithms = vec![
        CompressionType::Gzip,
        CompressionType::Deflate,
        CompressionType::Brotli,
        CompressionType::Zstd,
    ];
    
    // Pre-compress test data
    let test_data = create_test_data(102_400, "text");
    let mut compressor = ResponseCompressor::new();
    let mut compressed_data = std::collections::HashMap::new();
    
    for algorithm in &algorithms {
        let compressed = compressor.compress(&test_data, algorithm.clone()).unwrap();
        compressed_data.insert(algorithm, compressed);
    }
    
    for algorithm in &algorithms {
        if let Some(compressed) = compressed_data.get(algorithm) {
            group.throughput(Throughput::Bytes(test_data.len() as u64));
            
            group.bench_with_input(
                BenchmarkId::new("decompression", format!("{:?}", algorithm)),
                compressed,
                |b, data| {
                    let mut compressor = ResponseCompressor::new();
                    b.iter(|| {
                        black_box(compressor.decompress(data, algorithm.clone()).unwrap())
                    })
                },
            );
        }
    }
    
    group.finish();
}

fn bench_compression_levels(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_levels");
    
    let test_data = create_test_data(100_000, "text");
    let levels = vec![1, 3, 6, 9];
    let algorithms = vec![CompressionType::Gzip, CompressionType::Brotli];
    
    for algorithm in &algorithms {
        for level in &levels {
            group.bench_with_input(
                BenchmarkId::new(
                    format!("{:?}_level", algorithm),
                    level
                ),
                &test_data,
                |b, data| {
                    let mut compressor = ResponseCompressor::new().with_level(*level);
                    b.iter(|| {
                        black_box(compressor.compress(data, algorithm.clone()).unwrap())
                    })
                },
            );
        }
    }
    
    group.finish();
}

fn bench_streaming_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_compression");
    
    let total_size = 1_048_576; // 1MB
    let chunk_sizes = vec![1_024, 4_096, 16_384, 65_536]; // 1KB to 64KB chunks
    let algorithms = vec![CompressionType::Gzip, CompressionType::Brotli];
    
    for algorithm in &algorithms {
        for chunk_size in &chunk_sizes {
            let test_data = create_test_data(total_size, "text");
            
            group.bench_with_input(
                BenchmarkId::new(
                    format!("{:?}_streaming", algorithm),
                    chunk_size
                ),
                &test_data,
                |b, data| {
                    b.iter(|| {
                        let mut compressor = StreamingCompressor::new(algorithm.clone(), *chunk_size);
                        
                        for chunk in data.chunks(*chunk_size) {
                            black_box(compressor.write(chunk).unwrap());
                        }
                        
                        black_box(compressor.finish().unwrap())
                    })
                },
            );
        }
    }
    
    group.finish();
}

fn bench_compression_response(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_response");
    
    let sizes = vec![10_240, 102_400]; // 10KB, 100KB
    let content_types = vec!["text/html", "application/json", "text/css"];
    let accept_encodings = vec![
        "gzip",
        "gzip, deflate",
        "gzip, deflate, br",
        "gzip, deflate, br, zstd",
    ];
    
    for size in &sizes {
        for content_type in &content_types {
            let test_data = create_test_data(*size, "text");
            
            for accept_encoding in &accept_encodings {
                group.bench_with_input(
                    BenchmarkId::new(
                        format!("response_{}_{}", content_type.replace("/", "_"), size),
                        accept_encoding.len()
                    ),
                    &test_data,
                    |b, data| {
                        let mut compressor = ResponseCompressor::new();
                        b.iter(|| {
                            black_box(compressor.compress_response(
                                data,
                                content_type,
                                accept_encoding
                            ))
                        })
                    },
                );
            }
        }
    }
    
    group.finish();
}

fn bench_middleware_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("middleware_processing");
    
    let sizes = vec![1_024, 10_240, 102_400]; // 1KB, 10KB, 100KB
    let test_data_map: std::collections::HashMap<usize, Vec<u8>> = sizes.iter()
        .map(|&size| (size, create_test_data(size, "html")))
        .collect();
    
    for size in &sizes {
        if let Some(test_data) = test_data_map.get(size) {
            group.throughput(Throughput::Bytes(*size as u64));
            
            group.bench_with_input(
                BenchmarkId::new("middleware", size),
                test_data,
                |b, data| {
                    let mut middleware = CompressionMiddleware::new();
                    
                    b.iter(|| {
                        let mut headers = std::collections::HashMap::new();
                        headers.insert("Content-Type".to_string(), "text/html".to_string());
                        headers.insert("Accept-Encoding".to_string(), "gzip, br".to_string());
                        
                        black_box(middleware.process_response(data, &mut headers))
                    })
                },
            );
        }
    }
    
    group.finish();
}

fn bench_compression_selection(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_selection");
    
    let accept_encodings = vec![
        "gzip",
        "gzip, deflate",
        "gzip, deflate, br",
        "gzip;q=0.8, deflate;q=0.6, br;q=1.0, zstd;q=0.9",
        "gzip;q=0.8, deflate;q=0.6, br;q=1.0, zstd;q=0.9, *;q=0.1",
    ];
    
    let compressor = ResponseCompressor::new();
    
    for (i, accept_encoding) in accept_encodings.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("selection", i),
            accept_encoding,
            |b, encoding| {
                b.iter(|| {
                    black_box(compressor.select_compression(encoding))
                })
            },
        );
    }
    
    group.finish();
}

fn bench_statistics_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("statistics_overhead");
    
    let test_data = create_test_data(10_240, "text");
    
    // Benchmark with statistics enabled (default)
    group.bench_function("with_stats", |b| {
        let mut compressor = ResponseCompressor::new();
        b.iter(|| {
            black_box(compressor.compress(&test_data, CompressionType::Gzip).unwrap())
        })
    });
    
    // For comparison, benchmark the core compression without our wrapper
    group.bench_function("raw_gzip", |b| {
        use flate2::{Compression, write::GzEncoder};
        use std::io::Write;
        
        b.iter(|| {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::new(6));
            encoder.write_all(&test_data).unwrap();
            black_box(encoder.finish().unwrap())
        })
    });
    
    group.finish();
}

fn bench_concurrent_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_compression");
    
    let test_data = create_test_data(50_000, "text");
    let thread_counts = vec![1, 2, 4, 8];
    
    for thread_count in &thread_counts {
        group.bench_with_input(
            BenchmarkId::new("concurrent", thread_count),
            &test_data,
            |b, data| {
                b.iter(|| {
                    let data = data.clone();
                    let mut handles = vec![];
                    
                    for _ in 0..*thread_count {
                        let data = data.clone();
                        let handle = std::thread::spawn(move || {
                            let mut compressor = ResponseCompressor::new();
                            compressor.compress(&data, CompressionType::Gzip).unwrap()
                        });
                        handles.push(handle);
                    }
                    
                    for handle in handles {
                        black_box(handle.join().unwrap());
                    }
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    name = compression_benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(50);
    targets = 
        bench_compression_algorithms,
        bench_decompression_algorithms,
        bench_compression_levels,
        bench_streaming_compression,
        bench_compression_response,
        bench_middleware_processing,
        bench_compression_selection,
        bench_statistics_overhead,
        bench_concurrent_compression
);

criterion_main!(compression_benches);
