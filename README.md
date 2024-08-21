# ActionExecutor Contract Documentation

The `ActionExecutor` contract is designed to facilitate the execution of multiple operations sequentially in a single transaction.
It works by building up transaction state information in memory and using it when certain operations (like `CALL`, `CREATE`, or `DELEGATECALL`) are executed.  This design allows for flexible execution of a series of contract interactions, with the transaction context being gradually constructed and modified by various setup operations before being executed by a call or creation.


## Core Operations

The operations are encoded as single-byte opcodes, and each operation modifies the in-memory transaction state. The following are the operations supported:

- **CLEARDATA (0x00):** Clears the in-memory transaction data (`txData`) and sets its size.
- **SETDATA (0x01):** Sets chunks of data into `txData`, modifying specific segments.
- **SETADDR (0x02):** Sets the target address to be used in the next `CALL`, `CREATE`, or `DELEGATECALL`.
- **SETVALUE (0x03):** Sets the value (ETH) to be sent in the next transaction.
- **EXTCODECOPY (0x04):** Copies the external code of a contract into the `txData`.
- **CALL (0x05):** Executes a contract call using the built-up in-memory transaction state (address, data, and value).
- **CREATE (0x06):** Deploys a new contract using the built-up in-memory transaction state (data and value), and stores the result address in the memory state.
- **DELEGATECALL (0x07):** DEBUG. Executes a delegatecall to another contract, using the accumulated transaction state.

## Memory Context

The contract accumulates tx info state in memory, represented by:
- `target`: The address that will be used for calls.
- `value`: The amount of ETH to send with the call.
- `txData`: The calldata to send with the call or contract creation.

Operations like `SETDATA`, `SETADDR`, and `SETVALUE` prepare this state, and then `CALL`, `CREATE`, or `DELEGATECALL` use it.

## Opcode Documentation

| Opcode  | Operation      | Encoding Details                                                  
|---------|----------------|-------------------------------------------------------------------
| 0x00    | CLEARDATA      | `0x00 + [size: uint16]`                                           
| 0x01    | SETDATA        | `0x01 + [data_offset: uint16] + [count: uint16] + [32 bytes values...]`     
| 0x02    | SETADDR        | `0x02 + [address: bytes20]`                                       
| 0x03    | SETVALUE       | `0x03 + [value: uint256]`                                         
| 0x04    | EXTCODECOPY    | `0x04 + [source_address: bytes20] + [dest_offset: uint16] + [code_offset: uint16] + [size: uint16]` 
| 0x05    | CALL           | `0x05`                                                           
| 0x06    | CREATE         | `0x06`                                                           
| 0x07    | DELEGATECALL   | `0x07`                                                           

## Example Use Case

Consider a sequence where the contract clears the transaction data, sets an address and value, populates some data, and then performs a call.

```text
0x00 + 0x0010             // CLEARDATA (size = 16 bytes)
0x02 + <address>          // SETADDR (target address)
0x03 + <value>            // SETVALUE (send ETH)
0x01 + 0x0000 + 0x0002    // SETDATA (offset = 0, 2 items)
<32 bytes of data item 1> // Data chunk 1
<32 bytes of data item 2> // Data chunk 2
0x80                      // CALL (executes the call using prepared state)
```

This sequence builds up the txData and sets up a target address and value before making the contract call.


## Setup & Execution

1. Deployment

When the contract is deployed, the owner is set to the deployer address.

2. Operation Flow

The operations are triggered through the fallback function, where the contract receives encoded data. This data is decoded and processed step by step to execute various actions.

3. Memory Transaction Context

The memory transaction context is built up before certain operations like CALL, CREATE, or DELEGATECALL are triggered.



# ActionExecutorProxy Contract Documentation

`ActionExecutorProxy` is a proxy contract designed to delegate all of its operations to a target contract (`ActionExecutor`). The primary purpose of this contract is to streamline the execution of a set of pre-defined actions (encoded as `constructorData`) in a single transaction during the deployment of the proxy contract. The proxy will then delegate these operations to an already deployed instance of `ActionExecutor`.

	•	Single Transaction Execution: This proxy allows you to bundle multiple actions into one atomic transaction during deployment.
	•	Pre-Deployed ActionExecutor: Ensure that the ActionExecutor contract is deployed and its address is known before deploying the proxy.
	•	Owner-Only Access: The proxy enforces an ownership check, allowing only the deployer (owner) to interact with it after deployment.

## Usage

1. **Deploy `ActionExecutor`:**
   First, deploy the `ActionExecutor` contract. Make sure to note the address of the deployed contract, as you will need this address when deploying the proxy contract.

2. **Deploy `ActionExecutorProxy`:**
   Deploy `ActionExecutorProxy` with the following parameters:
   - `target`: The address of the pre-deployed `ActionExecutor` contract.
   - `constructorData`: A sequence of encoded operations to be executed by the `ActionExecutor`.

   The `constructorData` will be passed through a `delegatecall` to the `ActionExecutor` and executed immediately upon the proxy's deployment.

3. **Ownership**:
   Only the owner (the address that deployed the proxy) is allowed to send subsequent transactions to the proxy. All calls are delegated to the `ActionExecutor`.

4. **Fallback Function**:
   After deployment, any calls sent to the proxy will be forwarded to the `ActionExecutor` via `delegatecall`. The proxy does not maintain any logic on its own—everything is handled by the `ActionExecutor`.

