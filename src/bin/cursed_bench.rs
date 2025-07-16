use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, exit};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json;

/// CURSED Benchmark CLI Tool
/// 
/// Provides a command-line interface for running CURSED benchmarks,
/// analyzing performance, and generating reports.

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchmarkConfig {
    name: String,
    source_file: String,
    warmup_iterations: u32,
    measurement_iterations: u32,
    timeout_seconds: u32,
    memory_tracking: bool,
    cpu_tracking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BenchmarkResult {
    name: String,
    execution_times: Vec<f64>,
    memory_usage: Vec<u64>,
    cpu_usage: Vec<f64>,
    mean_time: f64,
    median_time: f64,
    std_dev: f64,
    min_time: f64,
    max_time: f64,
    total_iterations: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComparisonResult {
    baseline_name: String,
    current_name: String,
    performance_change: f64,
    is_regression: bool,
    significance_level: f64,
}

struct BenchmarkRunner {
    config: BenchmarkConfig,
    results: Vec<BenchmarkResult>,
}

impl BenchmarkRunner {
    fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    fn run_benchmark(&mut self, source_file: &str) -> Result<BenchmarkResult, String> {
        println!("Running benchmark: {}", self.config.name);
        
        // Compile the CURSED program
        let compile_result = Command::new("cargo")
            .args(&["run", "--bin", "cursed", "--", "compile", source_file])
            .output()
            .map_err(|e| format!("Failed to compile benchmark: {}", e))?;

        if !compile_result.status.success() {
            return Err(format!("Compilation failed: {}", 
                String::from_utf8_lossy(&compile_result.stderr)));
        }

        let executable_name = Path::new(source_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("benchmark");

        // Run warmup iterations
        println!("Running {} warmup iterations...", self.config.warmup_iterations);
        for _ in 0..self.config.warmup_iterations {
            let _ = Command::new(format!("./{}", executable_name))
                .output()
                .map_err(|e| format!("Failed to run warmup: {}", e))?;
        }

        // Run measurement iterations
        println!("Running {} measurement iterations...", self.config.measurement_iterations);
        let mut execution_times = Vec::new();
        let mut memory_usage = Vec::new();
        let mut cpu_usage = Vec::new();

        for i in 0..self.config.measurement_iterations {
            let start_time = Instant::now();
            let start_memory = self.get_memory_usage();
            let start_cpu = self.get_cpu_usage();

            let output = Command::new(format!("./{}", executable_name))
                .output()
                .map_err(|e| format!("Failed to run benchmark iteration: {}", e))?;

            let end_time = Instant::now();
            let end_memory = self.get_memory_usage();
            let end_cpu = self.get_cpu_usage();

            if !output.status.success() {
                return Err(format!("Benchmark execution failed: {}", 
                    String::from_utf8_lossy(&output.stderr)));
            }

            let duration = end_time.duration_since(start_time);
            execution_times.push(duration.as_secs_f64());
            memory_usage.push(end_memory.saturating_sub(start_memory));
            cpu_usage.push(end_cpu - start_cpu);

            if i % 10 == 0 {
                println!("Completed iteration: {}/{}", i + 1, self.config.measurement_iterations);
            }
        }

        // Calculate statistics
        let stats = self.calculate_statistics(&execution_times);

        let result = BenchmarkResult {
            name: self.config.name.clone(),
            execution_times,
            memory_usage,
            cpu_usage,
            mean_time: stats.0,
            median_time: stats.1,
            std_dev: stats.2,
            min_time: stats.3,
            max_time: stats.4,
            total_iterations: self.config.measurement_iterations,
        };

        self.results.push(result.clone());
        Ok(result)
    }

    fn calculate_statistics(&self, values: &[f64]) -> (f64, f64, f64, f64, f64) {
        if values.is_empty() {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }

        let sum: f64 = values.iter().sum();
        let mean = sum / values.len() as f64;

        let mut sorted_values = values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median = if sorted_values.len() % 2 == 0 {
            let mid = sorted_values.len() / 2;
            (sorted_values[mid - 1] + sorted_values[mid]) / 2.0
        } else {
            sorted_values[sorted_values.len() / 2]
        };

        let variance: f64 = values
            .iter()
            .map(|value| {
                let diff = mean - value;
                diff * diff
            })
            .sum::<f64>() / values.len() as f64;

        let std_dev = variance.sqrt();
        let min_time = *sorted_values.first().unwrap();
        let max_time = *sorted_values.last().unwrap();

        (mean, median, std_dev, min_time, max_time)
    }

    fn get_memory_usage(&self) -> u64 {
        // Simplified memory usage - in production would use system APIs
        0
    }

    fn get_cpu_usage(&self) -> f64 {
        // Simplified CPU usage - in production would use system APIs
        0.0
    }

    fn format_result(&self, result: &BenchmarkResult) -> String {
        format!(
            "Benchmark Results for: {}\n\
             Mean Time: {:.6}s\n\
             Median Time: {:.6}s\n\
             Std Deviation: {:.6}s\n\
             Min Time: {:.6}s\n\
             Max Time: {:.6}s\n\
             Total Iterations: {}\n",
            result.name,
            result.mean_time,
            result.median_time,
            result.std_dev,
            result.min_time,
            result.max_time,
            result.total_iterations
        )
    }

    fn save_results(&self, filename: &str) -> Result<(), String> {
        let json_data = serde_json::to_string_pretty(&self.results)
            .map_err(|e| format!("Failed to serialize results: {}", e))?;

        fs::write(filename, json_data)
            .map_err(|e| format!("Failed to write results file: {}", e))?;

        Ok(())
    }

    fn load_previous_results(&self, filename: &str) -> Result<Vec<BenchmarkResult>, String> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read previous results: {}", e))?;

        let results: Vec<BenchmarkResult> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse previous results: {}", e))?;

        Ok(results)
    }

    fn generate_html_report(&self, filename: &str) -> Result<(), String> {
        let mut html = String::from(
            "<!DOCTYPE html>\n\
             <html>\n\
             <head>\n\
             <title>CURSED Benchmark Report</title>\n\
             <style>\n\
             body { font-family: Arial, sans-serif; margin: 20px; }\n\
             table { border-collapse: collapse; width: 100%; }\n\
             th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n\
             th { background-color: #f2f2f2; }\n\
             .regression { background-color: #ffebee; }\n\
             .improvement { background-color: #e8f5e8; }\n\
             </style>\n\
             </head>\n\
             <body>\n\
             <h1>CURSED Benchmark Report</h1>\n\
             <table>\n\
             <tr><th>Benchmark</th><th>Mean Time (s)</th><th>Std Dev (s)</th><th>Min (s)</th><th>Max (s)</th></tr>\n"
        );

        for result in &self.results {
            html.push_str(&format!(
                "<tr>\
                 <td>{}</td>\
                 <td>{:.6}</td>\
                 <td>{:.6}</td>\
                 <td>{:.6}</td>\
                 <td>{:.6}</td>\
                 </tr>\n",
                result.name,
                result.mean_time,
                result.std_dev,
                result.min_time,
                result.max_time
            ));
        }

        html.push_str("</table>\n</body>\n</html>");

        fs::write(filename, html)
            .map_err(|e| format!("Failed to write HTML report: {}", e))?;

        Ok(())
    }
}

fn print_usage() {
    println!(
        "CURSED Benchmark Tool\n\n\
         Usage: cursed_bench [OPTIONS] <command> [ARGS]\n\n\
         Commands:\n\
         \trun <file.csd>              Run benchmark on CURSED file\n\
         \tbatch <config.json>         Run multiple benchmarks from config\n\
         \tcompare <baseline> <current> Compare two benchmark results\n\
         \treport <results.json>       Generate HTML report\n\n\
         Options:\n\
         \t--warmup <n>               Number of warmup iterations (default: 100)\n\
         \t--iterations <n>           Number of measurement iterations (default: 1000)\n\
         \t--timeout <n>              Timeout in seconds (default: 60)\n\
         \t--output <file>            Output file for results\n\
         \t--name <name>              Benchmark name\n\
         \t--memory                   Track memory usage\n\
         \t--cpu                      Track CPU usage\n\
         \t--help                     Show this help message\n"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        exit(1);
    }

    let mut config = BenchmarkConfig {
        name: "default".to_string(),
        source_file: String::new(),
        warmup_iterations: 100,
        measurement_iterations: 1000,
        timeout_seconds: 60,
        memory_tracking: false,
        cpu_tracking: false,
    };

    let mut output_file = String::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--help" => {
                print_usage();
                exit(0);
            }
            "--warmup" => {
                i += 1;
                if i < args.len() {
                    config.warmup_iterations = args[i].parse().unwrap_or(100);
                }
            }
            "--iterations" => {
                i += 1;
                if i < args.len() {
                    config.measurement_iterations = args[i].parse().unwrap_or(1000);
                }
            }
            "--timeout" => {
                i += 1;
                if i < args.len() {
                    config.timeout_seconds = args[i].parse().unwrap_or(60);
                }
            }
            "--output" => {
                i += 1;
                if i < args.len() {
                    output_file = args[i].clone();
                }
            }
            "--name" => {
                i += 1;
                if i < args.len() {
                    config.name = args[i].clone();
                }
            }
            "--memory" => {
                config.memory_tracking = true;
            }
            "--cpu" => {
                config.cpu_tracking = true;
            }
            "run" => {
                i += 1;
                if i < args.len() {
                    config.source_file = args[i].clone();
                    
                    let mut runner = BenchmarkRunner::new(config);
                    
                    match runner.run_benchmark(&runner.config.source_file.clone()) {
                        Ok(result) => {
                            println!("{}", runner.format_result(&result));
                            
                            if !output_file.is_empty() {
                                if let Err(e) = runner.save_results(&output_file) {
                                    eprintln!("Failed to save results: {}", e);
                                    exit(1);
                                }
                                println!("Results saved to: {}", output_file);
                            }
                        }
                        Err(e) => {
                            eprintln!("Benchmark failed: {}", e);
                            exit(1);
                        }
                    }
                }
                break;
            }
            "report" => {
                i += 1;
                if i < args.len() {
                    let results_file = &args[i];
                    println!("Generating HTML report from: {}", results_file);
                    
                    // Create a dummy runner to use for report generation
                    let dummy_config = BenchmarkConfig {
                        name: "report".to_string(),
                        source_file: String::new(),
                        warmup_iterations: 0,
                        measurement_iterations: 0,
                        timeout_seconds: 0,
                        memory_tracking: false,
                        cpu_tracking: false,
                    };
                    let mut runner = BenchmarkRunner::new(dummy_config);
                    
                    match runner.load_previous_results(results_file) {
                        Ok(results) => {
                            runner.results = results;
                            let html_file = if output_file.is_empty() {
                                "report.html".to_string()
                            } else {
                                output_file.clone()
                            };
                            if let Err(e) = runner.generate_html_report(&html_file) {
                                eprintln!("Failed to generate HTML report: {}", e);
                                exit(1);
                            }
                            println!("HTML report generated: {}", html_file);
                        }
                        Err(e) => {
                            eprintln!("Failed to load results: {}", e);
                            exit(1);
                        }
                    }
                }
                break;
            }
            _ => {}
        }
        i += 1;
    }
}
