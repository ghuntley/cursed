use std::env;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("CURSED Build System v1.0.0");
        eprintln!("Usage: cursed_build <command> [options]");
        eprintln!();
        eprintln!("Commands:");
        eprintln!("  build     Build the project");
        eprintln!("  test      Run all tests");
        eprintln!("  clean     Clean build artifacts");
        eprintln!("  rebuild   Clean and build");
        eprintln!("  install   Install a package");
        eprintln!("  list      List installed packages");
        eprintln!("  watch     Watch for changes and rebuild");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --config=<path>  Use custom build configuration file");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  cursed_build build");
        eprintln!("  cursed_build test");
        eprintln!("  cursed_build install json_parser 1.0.0");
        eprintln!("  cursed_build build --config=custom.toml");
        process::exit(1);
    }

    // Create a temporary CURSED script that calls the build system
    let build_script = r#"
yeet "build_system_simple"

slay main(args []tea) normie {
    damn build_system_simple.build_system_main_simple(args)
}
"#;

    // Write the script to a temporary file
    let temp_script_path = ".cursed_build_temp.csd";
    match std::fs::write(temp_script_path, build_script) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error creating temporary build script: {}", e);
            process::exit(1);
        }
    }

    // Prepare arguments for the CURSED interpreter
    let mut cursed_args = vec![
        "cargo".to_string(),
        "run".to_string(),
        "--bin".to_string(),
        "cursed".to_string(),
        "--".to_string(),
        temp_script_path.to_string(),
    ];

    // Add the original arguments (skip the first one which is the binary name)
    cursed_args.extend(args.iter().skip(1).cloned());

    // Execute the CURSED build system
    let output = Command::new("cargo")
        .args(&cursed_args[1..]) // Skip the "cargo" part since we're using Command::new("cargo")
        .current_dir(".")
        .output();

    // Clean up temporary file
    let _ = std::fs::remove_file(temp_script_path);

    match output {
        Ok(output) => {
            // Print stdout and stderr
            print!("{}", String::from_utf8_lossy(&output.stdout));
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
            
            // Exit with the same status code as the CURSED process
            process::exit(output.status.code().unwrap_or(1));
        }
        Err(e) => {
            eprintln!("Error executing CURSED build system: {}", e);
            eprintln!("Make sure the CURSED compiler is built and available.");
            eprintln!("Try running: cargo build --bin cursed");
            process::exit(1);
        }
    }
}
