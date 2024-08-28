use std::{fs::File, io::Write, process::Command};
use hex;

// Example custom build script.
fn main() {
 
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=contracts/executor.sol");
    println!("cargo::rerun-if-changed=contracts/proxy.sol");

    // compile the solidity files into executor.bin and proxy.bin
    let output_executor = Command::new("sh").args(["-c", "solc contracts/executor.sol --via-ir --optimize --optimize-runs 2000 --bin"]).output().unwrap();
    let mut file = File::create("contracts/executor.bin").unwrap();
    file.write_all(&hex::decode(output_executor.stdout[56..].trim_ascii()).unwrap()).unwrap();


    // compile the solidity files into executor.bin and proxy.bin
    let output_proxy = Command::new("sh").args(["-c", "solc contracts/proxy.sol --via-ir --optimize --optimize-runs 2000 --bin"]).output().unwrap();
    let mut file = File::create("contracts/proxy.bin").unwrap();
    file.write_all(&hex::decode(output_proxy.stdout[50..].trim_ascii()).unwrap()).unwrap();

    // print a warning if err
    if !output_executor.status.success() || !output_proxy.status.success() {
        println!("cargo::warning=Can NOT compile the solidity contracts. Make sure you have a solc compiler in the path.");
    }
}