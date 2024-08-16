// Contract initcode
pub const EXECUTOR_INIT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/executor.bin"));
pub const DELEGATE_PROXY_INIT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/proxy.bin"));

// Contract runtime code
pub const EXECUTOR_RUNTIME: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/executor_runtime.bin"));
pub const DELEGATE_PROXY_RUNTIME: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/proxy_runtime.bin"));

pub mod flow_builder;
pub mod opcodes;

// Re-export Flowbuilder
pub use flow_builder::FlowBuilder;

#[cfg(test)]
mod test;
