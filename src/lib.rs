// Contract initcode & runtime code

// Use include_bytes! when not building on docs.rs
#[cfg(not(docsrs))]
pub const EXECUTOR_INIT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/executor.bin"));
#[cfg(not(docsrs))]
pub const DELEGATE_PROXY_INIT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/proxy.bin"));
#[cfg(not(docsrs))]
pub const EXECUTOR_RUNTIME: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/executor_runtime.bin"));
#[cfg(not(docsrs))]
pub const DELEGATE_PROXY_RUNTIME: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/proxy_runtime.bin"));

// Use empty slices when building on docs.rs
#[cfg(docsrs)]
pub const EXECUTOR_INIT: &[u8] = &[];
#[cfg(docsrs)]
pub const DELEGATE_PROXY_INIT: &[u8] = &[];
#[cfg(docsrs)]
pub const EXECUTOR_RUNTIME: &[u8] = &[];
#[cfg(docsrs)]
pub const DELEGATE_PROXY_RUNTIME: &[u8] = &[];


pub mod flow_builder;
pub mod opcodes;

// Re-export Flowbuilder
pub use flow_builder::FlowBuilder;

#[cfg(test)]
mod test;
