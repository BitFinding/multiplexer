<div align="center">
  <img src=".github/assets/logo.png" alt="Executor Contract Logo" width="200"/>

# Multiplexer

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/multiplexer-evm.svg)](https://crates.io/crates/multiplexer-evm)
[![docs.rs](https://img.shields.io/docsrs/multiplexer-evm)](https://docs.rs/multiplexer-evm)
[![Rust](https://github.com/BitFinding/multiplexer/actions/workflows/rust.yml/badge.svg)](https://github.com/BitFinding/multiplexer/actions/workflows/rust.yml)

Originally developed as an internal MEV launchpad, Multiplexer is now open-sourced. It provides a flexible smart contract system for executing complex transaction sequences, featuring a Solidity executor contract and a Rust library for building execution flows.

</div>

> ⚠️ **WARNING**: This contract has not been audited. Using this contract with real assets could result in permanent loss of funds. Use at your own risk.

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/BitFinding/multiplexer.git
cd multiplexer

# Install dependencies and compile contracts
# (This uses build.rs to compile contracts/executor.sol and contracts/proxy.sol)
cargo build

# Run tests (requires a mainnet fork RPC URL)
ETH_RPC_URL=https://eth-mainnet.alchemyapi.io/v2/YOUR_API_KEY cargo test
```

## Architecture

The system consists of:

1.  **`executor.sol`**: The core contract that executes sequences of operations based on provided bytecode. It manages memory (`txData`) and handles callbacks.
2.  **`proxy.sol`**: A simple immutable proxy contract used to deploy the executor logic, allowing for potential future upgrades (though the current proxy is basic).
3.  **Rust Library (`src/`)**: Provides a `FlowBuilder` utility to easily construct the bytecode sequences for the executor, abstracting away the low-level opcode details.

## Example Usage

### Morpho Flash Loan Example (Rust FlowBuilder)

This example demonstrates how to construct the bytecode for a Morpho flash loan using the Rust `FlowBuilder`. The goal is to borrow 100 WETH, and the callback data (`inner_flow_bytes`) will contain the instructions to approve the repayment to Morpho.

```rust
use alloy::{
    primitives::{address, uint, Address, Bytes, U256, hex},
    sol,
    sol_types::{SolCall},
};
use multiplexer::FlowBuilder;

// Define necessary constants
const ONEHUNDRED_ETH: U256 = uint!(100000000000000000000_U256); // 100e18
const WETH9: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
const MORPHO: Address = address!("BBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb");

// Simplified Sol definitions for the example
sol! {
    interface IERC20 {
        function approve(address spender, uint256 value) external returns (bool);
    }
    interface IMorpho {
        function flashLoan(address token, uint256 assets, bytes calldata data) external;
    }
}

/// Generates the bytecode for a Morpho flash loan flow.
/// The flow borrows 100 WETH and sets up the callback to approve repayment.
fn generate_morpho_flashloan_flow() -> Vec<u8> {
    // 1. Prepare the inner flow (callback data): Approve WETH repayment.
    // This flow will be executed by the executor when Morpho calls back into it.
    // It needs to ensure Morpho can pull the funds back.
    let approve_calldata = IERC20::approveCall {
        spender: MORPHO,
        value: ONEHUNDRED_ETH, // Approve the exact loan amount.
                               // Note: A real flash loan requires approving amount + fee.
                               // The executor must hold sufficient WETH *before* this approval runs.
    }.abi_encode();

    let inner_flow_bytes = FlowBuilder::empty()
        .call(WETH9, &approve_calldata, U256::ZERO) // Call WETH9.approve(MORPHO, amount)
        .optimize()
        .build_raw(); // Get the raw bytecode for the inner flow.
                      // `build_raw()` produces *only* the sequence of action opcodes.

    // 2. Prepare the outer flow: Initiate the flash loan.
    // This is the main flow sent to the executor contract transaction.
    let flashloan_calldata = IMorpho::flashLoanCall {
        token: WETH9,                  // Asset to borrow
        assets: ONEHUNDRED_ETH,        // Amount to borrow
        data: inner_flow_bytes.into(), // Pass the repayment flow as callback data
    }.abi_encode();

    let main_flow_bytes = FlowBuilder::empty()
        .set_fail() // Revert the entire transaction if any subsequent call fails (including the callback)
        .set_callback(MORPHO) // Set Morpho as the expected callback address. The executor will only
                              // execute the callback data if msg.sender matches this address.
        .call(MORPHO, &flashloan_calldata, U256::ZERO) // Call Morpho.flashLoan(...)
        .optimize() // Apply peephole optimizations
        .build(); // Build the final bytecode sequence.
                  // `build()` prepends the `executeActions()` function selector (0xc94f554d)
                  // to the raw action bytecode generated by `build_raw()`.

    main_flow_bytes
}

// --- How to use the generated bytecode ---
fn main() {
    let flow_bytecode = generate_morpho_flashloan_flow();

    // `flow_bytecode` now contains the sequence:
    // SETFAIL -> SETCALLBACK(MORPHO) -> CALL(MORPHO, flashLoan(...))

    // This `flow_bytecode` would be used as the `data` field in an Ethereum transaction
    // sent to your deployed Executor contract instance.

    // Important Considerations for Execution:
    // 1. Funding: The Executor contract must possess enough WETH *after* the flash loan
    //    is granted but *before* the callback completes to successfully execute the
    //    `inner_flow_bytes` (the WETH approval) and allow Morpho to reclaim the funds + fee.
    //    This usually means the Executor needs some initial WETH or the operations *within*
    //    the flash loan (not shown in this basic example) must generate the required WETH.
    // 2. Gas: Ensure sufficient gas is provided for the main transaction and the callback execution.
    // 3. Permissions: The transaction sender must be the owner of the Executor contract.

    println!("Generated Flow Bytecode: 0x{}", hex::encode(&flow_bytecode));
}
```

### Low-Level Bytecode Example

Here's an example sequence that performs a basic contract call using the raw opcodes:

```text
# Assume target_addr = 0x... target contract address
# Assume eth_value = 1 ether (in wei)
# Assume function selector = 0xaabbccdd

# Bytecode Sequence:
0x01 0x0040          # CLEARDATA: Allocate 64 bytes for calldata
0x03 <target_addr>   # SETADDR: Set target contract address
0x04 <eth_value>     # SETVALUE: Set ETH value to send (1 ether)
0x02 0x0000 0x0004   # SETDATA: Set function selector at offset 0, length 4
aabbccdd             #   ↳ Function selector bytes
0x0A                 # SETFAIL: Enable revert on failure
0x06                 # CALL: Execute the call
0x00                 # EOF: End sequence
```

## Core Features

- Sequential execution of multiple operations in a single transaction
- Support for flash loans from multiple protocols (Morpho, Aave)
- Low-level operation support (calls, creates, delegate calls)
- Memory management for transaction data
- Fail-safe mechanisms with configurable error handling

## Operations

The contract supports the following operations, encoded as single-byte opcodes:

| Opcode | Operation    | Description                          | Encoding Format                                                                         |
| ------ | ------------ | ------------------------------------ | --------------------------------------------------------------------------------------- |
| 0x00   | EOF          | End of flow marker                   | `0x00`                                                                                  |
| 0x01   | CLEARDATA    | Clear transaction data buffer        | `0x01 + [size: uint16]`                                                                 |
| 0x02   | SETDATA      | Set data at specific offset          | `0x02 + [offset: uint16] + [size: uint16] + [ bytes]`                                   |
| 0x03   | SETADDR      | Set target address                   | `0x03 + [address: bytes20]`                                                             |
| 0x04   | SETVALUE     | Set ETH value for calls              | `0x04 + [value: uint256]`                                                               |
| 0x05   | EXTCODECOPY  | Copy external contract code          | `0x05 + [addr: bytes20] + [dataOffset: uint16] + [codeOffset: uint16] + [size: uint16]` |
| 0x06   | CALL         | Perform external call                | `0x06`                                                                                  |
| 0x07   | CREATE       | Deploy new contract                  | `0x07`                                                                                  |
| 0x08   | DELEGATECALL | Perform delegate call                | `0x08`                                                                                  |
| 0x09   | SETCALLBACK  | Set callback address for flash loans | `0x09 + [address: bytes20]`                                                             |
| 0x0A   | SETFAIL      | Enable revert on call failure        | `0x0A`                                                                                  |
| 0x0B   | CLEARFAIL    | Disable revert on call failure       | `0x0B`                                                                                  |

## Memory Management

The contract maintains a dynamic bytes array (`txData`) as a working buffer for all operations:

- Memory Layout:
  - 0x00-0x20: Length of array (32 bytes)
  - 0x20-onwards: Actual data bytes

Operations that interact with this buffer:

- CLEARDATA: Clears and resizes the buffer
- SETDATA: Writes data at specific offsets
- EXTCODECOPY: Copies external contract code into the buffer
- CALL/DELEGATECALL/CREATE: Read from the buffer for execution

## Flash Loan Support

The contract implements callbacks for multiple flash loan protocols:

### Morpho Flash Loan Callback

```solidity
function onMorphoFlashLoan(uint256 amount, bytes calldata data)
```

When Morpho calls this function on the executor, the executor will execute the bytecode passed in `data`.

### Aave Flash Loan Callback

```solidity
function executeOperation(
    address asset,
    uint256 amount,
    uint256 premium,
    address initiator,
    bytes calldata params
)
```

## Security Considerations

- Owner-only access control
- Callback address validation for flash loans (`SETCALLBACK`)
- Automatic callback address clearing after use (prevents re-entrancy with old callback data)
- Optional failure handling with `SETFAIL`/`CLEARFAIL`
- Memory bounds checking for all operations

## Development

The contract is developed in Solidity and includes a comprehensive test suite written in Rust. The tests use Anvil for local blockchain simulation.
