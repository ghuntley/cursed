use std::env;
use std::process::{Command, exit};
use std::time::Instant;

fn print_usage() {
    println!(
        "CURSED Benchmark Tool (Simple)\n\n\
         Usage: cursed_bench_simple [OPTIONS] <file.csd>\n\n\
         Options:\n\
         \t--iterations <n>     Number of iterations (default: 10)\n\
         \t--help              Show this help message\n"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        exit(1);
    }

    let mut iterations = 10;
    let mut source_file = String::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--help" => {
                print_usage();
                exit(0);
            }
            "--iterations" => {
                i += 1;
                if i < args.len() {
                    iterations = args[i].parse().unwrap_or(10);
                }
            }
            _ => {
                if !args[i].starts_with("--") {
                    source_file = args[i].clone();
                }
            }
        }
        i += 1;
    }

    if source_file.is_empty() {
        eprintln!("Error: No source file specified");
        exit(1);
    }

    println!("Running CURSED benchmark: {}", source_file);
    println!("Iterations: {}", iterations);

    let mut times = Vec::new();

    for i in 0..iterations {
        let start = Instant::now();
        
        let output = Command::new("cargo")
            .args(&["run", "--bin", "cursed", &source_file])
            .output()
            .expect("Failed to execute CURSED program");

        let duration = start.elapsed();
        times.push(duration.as_secs_f64());

        if !output.status.success() {
            eprintln!("Iteration {} failed: {}", i + 1, String::from_utf8_lossy(&output.stderr));
            continue;
        }

        println!("Iteration {}: {:.6}s", i + 1, duration.as_secs_f64());
    }

    if !times.is_empty() {
        let mean: f64 = times.iter().sum::<f64>() / times.len() as f64;
        let min = times.iter().copied().fold(f64::INFINITY, f64::min);
        let max = times.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        
        println!("\nBenchmark Results:");
        println!("Mean time: {:.6}s", mean);
        println!("Min time:  {:.6}s", min);
        println!("Max time:  {:.6}s", max);
        println!("Total iterations: {}", times.len());
    } else {
        eprintln!("No successful iterations completed");
        exit(1);
    }
}
