use std::path::Path;
use tracing::{instrument, info};
use crate::benchmark::language_comparison::{Language, Algorithm, run_language_benchmark, ensure_language_benchmarks};
use crate::benchmark::harness::{BenchmarkSuite, Benchmark, BenchmarkConfig};
use crate::benchmark::metrics::{Metric, TimingMetric};

#[instrument(skip_all, fields(suite_name = "expanded_language_comparison"))]
pub fn expanded_language_comparison_suite() -> BenchmarkSuite {
    info!("Creating expanded language comparison benchmark suite");
    
    let mut suite = BenchmarkSuite::new("expanded_language_comparison", "Expanded language comparison benchmark suite");
    
    // Add benchmarks for standard languages first (already implemented)
    add_language_benchmarks(&mut suite, Language::Cursed);
    add_language_benchmarks(&mut suite, Language::Rust);
    add_language_benchmarks(&mut suite, Language::C);
    add_language_benchmarks(&mut suite, Language::CSharp);
    add_language_benchmarks(&mut suite, Language::Go);
    add_language_benchmarks(&mut suite, Language::Java);
    add_language_benchmarks(&mut suite, Language::JavaScript);
    add_language_benchmarks(&mut suite, Language::PHP);
    add_language_benchmarks(&mut suite, Language::Perl);
    
    // Now add benchmarks for expanded languages
    add_language_benchmarks(&mut suite, Language::Haskell);
    add_language_benchmarks(&mut suite, Language::Swift);
    add_language_benchmarks(&mut suite, Language::Pascal);
    add_language_benchmarks(&mut suite, Language::OCaml);
    add_language_benchmarks(&mut suite, Language::Clojure);
    add_language_benchmarks(&mut suite, Language::CPlusPlus);
    add_language_benchmarks(&mut suite, Language::Zig);
    add_language_benchmarks(&mut suite, Language::Erlang);
    add_language_benchmarks(&mut suite, Language::Fortran);
    add_language_benchmarks(&mut suite, Language::Ruby);
    add_language_benchmarks(&mut suite, Language::Kotlin);
    
    suite
}

#[instrument(skip_all, fields(language = ?language))]
fn add_language_benchmarks(suite: &mut BenchmarkSuite, language: Language) {
    // Skip languages that may not be fully implemented yet
    if !ensure_language_benchmarks(language) {
        return;
    }
    
    // Add benchmarks for each algorithm
    add_benchmark(suite, language, Algorithm::BinaryTrees);
    add_benchmark(suite, language, Algorithm::NBodies);
    add_benchmark(suite, language, Algorithm::Mandelbrot);
    add_benchmark(suite, language, Algorithm::Fannkuch);
    add_benchmark(suite, language, Algorithm::Fasta);
    add_benchmark(suite, language, Algorithm::StringProcessing);
}

#[instrument(skip_all, fields(language = ?language, algorithm = ?algorithm))]
fn add_benchmark(suite: &mut BenchmarkSuite, language: Language, algorithm: Algorithm) {
    // Skip if benchmark file doesn't exist
    if !benchmark_file_exists(language, algorithm) {
        return;
    }
    
    let name = format!("{:?}_{:?}", language, algorithm);
    let description = format!("{:?} algorithm in {:?}", algorithm, language);
    let name_for_metric = name.clone();
    
    let benchmark = Benchmark::new(&name, &description, move || {
        // Run the benchmark and wrap the result as metrics
        let (duration, output) = run_language_benchmark(language, algorithm);
        vec![Box::new(TimingMetric { name: name_for_metric.clone(), duration }) as Box<dyn Metric>]
    })
    .with_config(BenchmarkConfig {
        iterations: 5,
        warmup: 1,
        ..Default::default()
    });
    
    suite.add_benchmark(benchmark);
}

// Check if a benchmark file exists for a given language and algorithm
fn benchmark_file_exists(language: Language, algorithm: Algorithm) -> bool {
    let file_extension = match language {
        Language::Cursed => "csd",
        Language::Rust => "rs",
        Language::C => "c",
        Language::CSharp => "cs",
        Language::Go => "go",
        Language::Java => "java",
        Language::JavaScript => "js",
        Language::PHP => "php",
        Language::Perl => "pl",
        Language::Haskell => "hs",
        Language::Swift => "swift",
        Language::Pascal => "pas",
        Language::OCaml => "ml",
        Language::Clojure => "clj",
        Language::CPlusPlus => "cpp",
        Language::Zig => "zig",
        Language::Erlang => "erl",
        Language::Fortran => "f90",
        Language::Ruby => "rb",
        Language::Kotlin => "kt",
        Language::Python => "py",
    };
    
    let filename = format!("{}.{}", algorithm.to_string(), file_extension);
    let path = Path::new("benchmarks").join(language.to_string().to_lowercase()).join(&filename);
    
    path.exists()
}