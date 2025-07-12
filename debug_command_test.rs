use std::process::Command;

fn main() {
    // Test llc command that doesn't exist in PATH
    let llc_result = Command::new("llc").arg("--version").output();
    println!("llc result: {:?}", llc_result);
    
    // Test existing llc command
    let llc_result2 = Command::new("/nix/store/vnxd8nqfibccfbczxwd9li5hw42k5kmw-llvm-19.1.6/bin/llc").arg("--version").output();
    println!("llc full path result: {:?}", llc_result2.is_ok());
    if let Ok(output) = llc_result2 {
        println!("llc status: {}", output.status.success());
    }
}
