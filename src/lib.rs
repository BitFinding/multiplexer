// Contract initcode
pub const EXECUTOR_INIT: &[u8] = include_bytes!("../contracts_output/executor.bin");
pub const DELEGATE_PROXY_INIT: &[u8] = include_bytes!("../contracts_output/proxy.bin");

// Contract runtime code
pub const EXECUTOR_RUNTIME: &[u8] = include_bytes!("../contracts_output/executor_runtime.bin");
pub const DELEGATE_PROXY_RUNTIME: &[u8] = include_bytes!("../contracts_output/proxy_runtime.bin");

pub mod opcodes;
pub mod flow_builder;

// Re-export Flowbuilder
pub use flow_builder::FlowBuilder;

#[cfg(test)]
mod test;
