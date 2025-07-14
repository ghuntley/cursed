use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed", "--", "compile", "--emit-llvm", "function_register_test.csd"])
        .output()
        .expect("Failed to execute command");

    println!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
}
