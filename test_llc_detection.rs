use std::process::Command;

fn main() {
    println!("Testing llc detection...");
    
    // Test the exact same logic as in the cursed compiler
    let llc_locations = vec![
        "llc".to_string(),
        "/nix/store/013b6qj9g2n2pmxcllnch9drrf9m0zwf-llvm-17.0.6/bin/llc".to_string(),
        "/nix/store/vnxd8nqfibccfbczxwd9li5hw42k5kmw-llvm-19.1.6/bin/llc".to_string(),
    ];
    
    for location in &llc_locations {
        println!("Trying llc at: {}", location);
        let llc_result = Command::new(location)
            .arg("--version")
            .output();
        
        if let Ok(output) = llc_result {
            if output.status.success() {
                println!("Found llc at: {}", location);
                println!("Version output: {}", String::from_utf8_lossy(&output.stdout));
                break;
            } else {
                println!("llc command failed at: {}", location);
            }
        } else {
            println!("llc not found at: {} (error: {:?})", location, llc_result.unwrap_err());
        }
    }
}
