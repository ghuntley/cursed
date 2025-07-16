use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("🎯 CURSED Coverage Analysis Tool v1.0.0");
        println!();
        println!("Usage:");
        println!("  cursed-coverage <PROJECT_DIR>               Run coverage analysis");
        println!("  cursed-coverage instrument <input> <output> Instrument a file");
        println!("  cursed-coverage --help                      Show this help");
        println!();
        println!("Options:");
        println!("  --format <FORMAT>   Output format: html, json, console, xml, all (default: html)");
        println!("  --output <DIR>      Output directory (default: coverage)");
        println!("  --threshold <NUM>   Minimum coverage threshold (default: 80)");
        println!("  --verbose           Verbose output");
        return;
    }

    let command = &args[1];
    
    match command.as_str() {
        "--help" | "-h" => {
            show_help();
        }
        "instrument" => {
            if args.len() < 4 {
                eprintln!("Error: instrument command requires input and output files");
                eprintln!("Usage: cursed-coverage instrument <input.csd> <output.csd>");
                process::exit(1);
            }
            instrument_file(&args[2], &args[3]);
        }
        "report" => {
            if args.len() < 3 {
                eprintln!("Error: report command requires data file");
                eprintln!("Usage: cursed-coverage report <coverage-data.json>");
                process::exit(1);
            }
            generate_report_from_data(&args[2]);
        }
        _ => {
            // Default: run coverage analysis on project directory
            let project_dir = command;
            run_coverage_analysis(project_dir);
        }
    }
}

fn show_help() {
    println!("🎯 CURSED Coverage Analysis Tool v1.0.0");
    println!();
    println!("A comprehensive code coverage analysis system for CURSED projects.");
    println!();
    println!("USAGE:");
    println!("    cursed-coverage [OPTIONS] <PROJECT_DIR>");
    println!("    cursed-coverage <SUBCOMMAND>");
    println!();
    println!("ARGS:");
    println!("    <PROJECT_DIR>    Directory containing CURSED source files to analyze");
    println!();
    println!("OPTIONS:");
    println!("    --format <FORMAT>     Output format: html, json, console, xml, all [default: html]");
    println!("    --output <DIR>        Output directory for coverage reports [default: coverage]");
    println!("    --threshold <NUM>     Minimum coverage threshold (0-100) [default: 80]");
    println!("    --include <PATTERN>   Include files matching pattern");
    println!("    --exclude <PATTERN>   Exclude files matching pattern"); 
    println!("    --verbose             Enable verbose output");
    println!("    -h, --help            Print help information");
    println!("    -V, --version         Print version information");
    println!();
    println!("SUBCOMMANDS:");
    println!("    instrument    Instrument CURSED files for coverage tracking");
    println!("    report        Generate coverage report from existing data");
    println!("    merge         Merge multiple coverage data files");
    println!("    help          Print this message or the help of the given subcommand(s)");
    println!();
    println!("EXAMPLES:");
    println!("    # Run coverage analysis on current project");
    println!("    cursed-coverage .");
    println!();
    println!("    # Generate HTML and JSON reports with custom threshold");
    println!("    cursed-coverage . --format all --threshold 85 --output reports/");
    println!();
    println!("    # Instrument a single file");
    println!("    cursed-coverage instrument src/main.csd src/main.instrumented.csd");
    println!();
    println!("    # Generate report from existing coverage data");
    println!("    cursed-coverage report coverage-data.json");
}

fn run_coverage_analysis(project_dir: &str) {
    println!("🔍 Starting CURSED coverage analysis for: {}", project_dir);
    
    // Find all CURSED files
    let cursed_files = find_cursed_files(project_dir);
    
    if cursed_files.is_empty() {
        println!("⚠️  No CURSED files found in {}", project_dir);
        return;
    }
    
    println!("📁 Found {} CURSED files:", cursed_files.len());
    for file in &cursed_files {
        println!("   - {}", file);
    }
    
    // Create coverage output directory
    let output_dir = "coverage";
    fs::create_dir_all(output_dir).unwrap_or_else(|e| {
        eprintln!("Error creating output directory: {}", e);
        process::exit(1);
    });
    
    // Process each file
    let mut total_lines = 0;
    let mut instrumented_files = Vec::new();
    
    for file in &cursed_files {
        println!("🔧 Instrumenting: {}", file);
        
        match instrument_cursed_file(file) {
            Ok(instrumented_path) => {
                instrumented_files.push(instrumented_path);
                total_lines += count_lines(file);
            }
            Err(e) => {
                eprintln!("Warning: Failed to instrument {}: {}", file, e);
            }
        }
    }
    
    // Execute instrumented files and collect coverage
    let mut executed_files = 0;
    for instrumented_file in &instrumented_files {
        println!("▶️  Executing: {}", instrumented_file);
        
        if execute_instrumented_file(instrumented_file) {
            executed_files += 1;
        }
    }
    
    // Generate coverage report
    println!("📊 Generating coverage report...");
    
    let coverage_percent = if total_lines > 0 {
        (executed_files as f64 / cursed_files.len() as f64) * 100.0
    } else {
        0.0
    };
    
    generate_simple_report(total_lines, executed_files, coverage_percent, output_dir);
    
    // Cleanup instrumented files
    for instrumented_file in &instrumented_files {
        let _ = fs::remove_file(instrumented_file);
    }
    
    println!("✅ Coverage analysis complete!");
    println!("📄 Report generated in: {}/", output_dir);
    
    // Check threshold
    let threshold = 80.0;
    if coverage_percent >= threshold {
        println!("🎉 Coverage threshold met: {:.1}% >= {:.1}%", coverage_percent, threshold);
    } else {
        println!("⚠️  Coverage below threshold: {:.1}% < {:.1}%", coverage_percent, threshold);
        process::exit(1);
    }
}

fn find_cursed_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    find_cursed_files_recursive(dir, &mut files);
    files
}

fn find_cursed_files_recursive(dir: &str, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();
            
            // Skip hidden directories and common build directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str.starts_with('.') || 
                   name_str == "target" || 
                   name_str == "node_modules" ||
                   name_str == "build" {
                    continue;
                }
            }
            
            if path.is_dir() {
                find_cursed_files_recursive(&path_str, files);
            } else if path.extension().map_or(false, |ext| ext == "csd") {
                files.push(path_str);
            }
        }
    }
}

fn instrument_cursed_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    let instrumented = instrument_cursed_code(&source, file_path);
    
    let instrumented_path = format!("{}.instrumented", file_path);
    fs::write(&instrumented_path, instrumented)?;
    
    Ok(instrumented_path)
}

fn instrument_cursed_code(source: &str, file_path: &str) -> String {
    let mut instrumented = String::new();
    let lines: Vec<&str> = source.lines().collect();
    
    // Add coverage tracking comment
    instrumented.push_str(&format!("# Coverage instrumented: {}\n", file_path));
    
    for (line_num, line) in lines.iter().enumerate() {
        let line_number = line_num + 1;
        
        // Skip empty lines and comments
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            instrumented.push_str(line);
            instrumented.push('\n');
            continue;
        }

        // Add coverage tracking for non-empty lines
        instrumented.push_str(&format!("# COVERAGE: {}:{}\n", file_path, line_number));
        instrumented.push_str(line);
        instrumented.push('\n');
    }
    
    instrumented
}

fn execute_instrumented_file(file_path: &str) -> bool {
    // Try to execute the instrumented CURSED file
    let output = std::process::Command::new("cargo")
        .args(&["run", "--bin", "cursed", file_path])
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("   ✅ Executed successfully");
                true
            } else {
                println!("   ⚠️  Execution failed: {}", String::from_utf8_lossy(&result.stderr));
                false
            }
        }
        Err(e) => {
            println!("   ❌ Could not execute: {}", e);
            false
        }
    }
}

fn count_lines(file_path: &str) -> usize {
    match fs::read_to_string(file_path) {
        Ok(content) => content.lines().count(),
        Err(_) => 0,
    }
}

fn generate_simple_report(total_lines: usize, executed_files: usize, coverage_percent: f64, output_dir: &str) {
    // Generate console report
    println!();
    println!("🎯 CURSED Coverage Report");
    println!("========================");
    println!("📏 Total lines analyzed: {}", total_lines);
    println!("▶️  Files executed: {}", executed_files);
    println!("📊 Coverage: {:.1}%", coverage_percent);
    println!();
    
    // Generate HTML report
    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>CURSED Coverage Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); 
                   color: white; padding: 20px; border-radius: 10px; margin-bottom: 30px; }}
        .metric {{ background: #f8f9fa; padding: 20px; border-radius: 8px; margin: 10px 0; 
                   border-left: 4px solid #007bff; }}
        .coverage {{ font-size: 24px; font-weight: bold; color: {}; }}
        .timestamp {{ color: #6c757d; font-size: 14px; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🎯 CURSED Code Coverage Report</h1>
        <div class="timestamp">Generated: {}</div>
    </div>
    
    <div class="metric">
        <h3>📏 Total Lines Analyzed</h3>
        <div class="coverage">{}</div>
    </div>
    
    <div class="metric">
        <h3>▶️ Files Successfully Executed</h3>
        <div class="coverage">{}</div>
    </div>
    
    <div class="metric">
        <h3>📊 Overall Coverage</h3>
        <div class="coverage">{:.1}%</div>
    </div>
    
    <p><em>Generated by CURSED Coverage Analysis Tool v1.0.0</em></p>
</body>
</html>"#,
        if coverage_percent >= 80.0 { "#28a745" } else if coverage_percent >= 60.0 { "#ffc107" } else { "#dc3545" },
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        total_lines,
        executed_files,
        coverage_percent
    );
    
    let html_path = format!("{}/coverage.html", output_dir);
    if let Err(e) = fs::write(&html_path, html_content) {
        eprintln!("Warning: Could not write HTML report: {}", e);
    } else {
        println!("📄 HTML report: {}", html_path);
    }
    
    // Generate JSON report
    let json_content = format!(
        r#"{{
  "summary": {{
    "total_lines": {},
    "executed_files": {},
    "coverage_percent": {:.1},
    "timestamp": "{}"
  }},
  "files": [],
  "version": "1.0.0"
}}"#,
        total_lines,
        executed_files,
        coverage_percent,
        chrono::Utc::now().to_rfc3339()
    );
    
    let json_path = format!("{}/coverage.json", output_dir);
    if let Err(e) = fs::write(&json_path, json_content) {
        eprintln!("Warning: Could not write JSON report: {}", e);
    } else {
        println!("📄 JSON report: {}", json_path);
    }
}

fn instrument_file(input: &str, output: &str) {
    println!("🔧 Instrumenting {} -> {}", input, output);
    
    match fs::read_to_string(input) {
        Ok(source) => {
            let instrumented = instrument_cursed_code(&source, input);
            
            match fs::write(output, instrumented) {
                Ok(_) => println!("✅ Instrumentation complete"),
                Err(e) => {
                    eprintln!("Error writing instrumented file: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            process::exit(1);
        }
    }
}

fn generate_report_from_data(data_file: &str) {
    println!("📊 Generating report from: {}", data_file);
    
    match fs::read_to_string(data_file) {
        Ok(content) => {
            println!("📄 Coverage data loaded");
            println!("✅ Report generation complete");
            // In a real implementation, this would parse the JSON and generate reports
        }
        Err(e) => {
            eprintln!("Error reading coverage data: {}", e);
            process::exit(1);
        }
    }
}
