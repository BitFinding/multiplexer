use std::process::Command;

// Example custom build script.
fn main() {
 
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=contracts/executor.sol");
    println!("cargo::rerun-if-changed=contracts/proxy.sol");

    // compile the solidity files into executor.bin and proxy.bin
    let status_executor = Command::new("sh").args(["-c", "solc contracts/executor.sol --overwrite --optimize --optimize-runs 200 --bin -o contracts/"]).status().unwrap();
    let status_proxy = Command::new("sh").args(["-c", "solc contracts/proxy.sol --overwrite --optimize --optimize-runs 200 --bin -o contracts/"]).status().unwrap();
    
    // print a warning if err
    if !status_executor.success() || !status_proxy.success() {
        println!("cargo::warning=Can NOT compile the solidity contracts. Make sure you have a solc compiler in the path.");
    }


}