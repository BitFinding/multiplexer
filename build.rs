use std::{collections::HashMap, fs::File, io::Write, process::Command};
use hex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SolSrcInfo {
    #[serde(rename = "bin-runtime")]
    bin_runtime: String,
    bin: String,
}

#[derive(Debug, Deserialize)]
struct SolJsonOut {
    contracts: HashMap<String, SolSrcInfo>,
}

fn build_get_json(source: &str) -> SolJsonOut {
    let output_execute = Command::new("sh").args(["-c", &format!("solc {} --via-ir --optimize --optimize-runs 2000 --combined-json=bin,bin-runtime", source)]).output().unwrap();
    let sol_output: SolJsonOut = serde_json::from_slice(&output_execute.stdout).expect("failed to load solc output");
    sol_output
}

// Example custom build script.
fn main() {
 
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=contracts/executor.sol");
    println!("cargo::rerun-if-changed=contracts/proxy.sol");

    let executor_file = "contracts/executor.sol";
    let executor_contract = "contracts/executor.sol:executor";
    let executor_solc = build_get_json(executor_file);
    let executor_outs = executor_solc.contracts.get(executor_contract).expect("solc output didn't generate the executor file");
    let executor_binruntime = &executor_outs.bin_runtime;
    let executor_bin = &executor_outs.bin;

    let mut file = File::create("contracts/executor.bin").unwrap();
    file.write_all(&hex::decode(executor_bin).expect("failed to decode hex binary from solc output")).unwrap();

    let mut file = File::create("contracts/executor_runtime.bin").unwrap();
    file.write_all(&hex::decode(executor_binruntime).expect("failed to decode hex binary from solc output")).unwrap();


    let proxy_file = "contracts/proxy.sol";
    let proxy_contract = "contracts/proxy.sol:proxy";
    let proxy_solc = build_get_json(proxy_file);
    let proxy_outs = proxy_solc.contracts.get(proxy_contract).expect("solc output didn't generate the proxy file");
    let proxy_binruntime = &proxy_outs.bin_runtime;
    let proxy_bin = &proxy_outs.bin;

    let mut file = File::create("contracts/proxy.bin").unwrap();
    file.write_all(&hex::decode(proxy_bin).expect("failed to decode hex binary from solc output")).unwrap();

    let mut file = File::create("contracts/proxy_runtime.bin").unwrap();
    file.write_all(&hex::decode(proxy_binruntime).expect("failed to decode hex binary from solc output")).unwrap();
}
