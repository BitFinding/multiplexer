use alloy_primitives::{Address, U256};

use crate::opcodes::*;

/// Function selector for `executeActions()`.
/// Derived from `keccak256("executeActions()")[..4]`.
/// Verified by `test_execute_actions_selector` in `test.rs`.
const EXECUTE_ACTIONS_SELECTOR: [u8; 4] = [0xc9, 0x4f, 0x55, 0x4d];

// ---------------------------------------------------------------------------
// Action enum — single source of truth for opcode encoding
// ---------------------------------------------------------------------------

/// A single operation in the executor bytecode stream.
///
/// Each variant maps 1:1 to an opcode constant in [`crate::opcodes`].
enum Action {
    ClearData {
        size: u16,
    },
    SetData {
        offset: u16,
        data: Vec<u8>,
    },
    SetAddr {
        addr: Address,
    },
    SetValue {
        value: U256,
    },
    ExtCodeCopy {
        source: Address,
        data_offset: u16,
        code_offset: u16,
        size: u16,
    },
    Call,
    /// `created_address` is **optimizer-only metadata**: it tells the peephole
    /// optimizer which address the CREATE will produce so it can elide a
    /// subsequent redundant `SetAddr`. It is *not* encoded into the bytecode.
    Create {
        created_address: Address,
    },
    DelegateCall,
    SetCallback {
        callback_address: Address,
    },
    SetFail,
    ClearFail,
}

impl Action {
    fn encode(&self) -> Vec<u8> {
        match self {
            Action::ClearData { size } => {
                let mut buf = vec![OP_CLEARDATA];
                buf.extend(&size.to_be_bytes());
                buf
            }
            Action::SetData { offset, data } => {
                let data_size = data.len() as u16;
                let mut buf = vec![OP_SETDATA];
                buf.extend(&offset.to_be_bytes());
                buf.extend(&data_size.to_be_bytes());
                buf.extend(data);
                buf
            }
            Action::SetAddr { addr } => {
                let mut buf = vec![OP_SETADDR];
                buf.extend(addr.as_slice());
                buf
            }
            Action::SetValue { value } => {
                let mut buf = vec![OP_SETVALUE];
                buf.extend(&value.to_be_bytes::<32>());
                buf
            }
            Action::ExtCodeCopy {
                source,
                data_offset,
                code_offset,
                size,
            } => {
                let mut buf = vec![OP_EXTCODECOPY];
                buf.extend(source.as_slice());
                buf.extend(&data_offset.to_be_bytes());
                buf.extend(&code_offset.to_be_bytes());
                buf.extend(&size.to_be_bytes());
                buf
            }
            Action::Call => vec![OP_CALL],
            Action::Create { .. } => vec![OP_CREATE],
            Action::DelegateCall => vec![OP_DELEGATECALL],
            Action::SetCallback { callback_address } => {
                let mut buf = vec![OP_SETCALLBACK];
                buf.extend(callback_address.as_slice());
                buf
            }
            Action::SetFail => vec![OP_SETFAIL],
            Action::ClearFail => vec![OP_CLEARFAIL],
        }
    }
}

// ---------------------------------------------------------------------------
// FlowBuilder
// ---------------------------------------------------------------------------

/// Builder for constructing executor bytecode action sequences.
///
/// Methods that add actions return `&mut Self` for chaining.
/// Call [`optimize`](Self::optimize) before [`build`](Self::build) to remove
/// redundant operations.
#[derive(Default)]
pub struct FlowBuilder {
    actions: Vec<Action>,
}

impl FlowBuilder {
    /// Creates an empty `FlowBuilder` with no actions.
    pub fn empty() -> Self {
        Self::default()
    }

    // -- Low-level opcode pushers ------------------------------------------

    /// Adds a `CLEARDATA` operation to the action list.
    pub fn set_cleardata_op(&mut self, size: u16) -> &mut Self {
        self.actions.push(Action::ClearData { size });
        self
    }

    /// Adds a `SETDATA` operation to the action list.
    pub fn set_data_op(&mut self, offset: u16, data: &[u8]) -> &mut Self {
        self.actions.push(Action::SetData {
            offset,
            data: data.to_owned(),
        });
        self
    }

    /// Adds a `SETADDR` operation to the action list.
    pub fn set_addr_op(&mut self, addr: Address) -> &mut Self {
        self.actions.push(Action::SetAddr { addr });
        self
    }

    /// Adds a `SETVALUE` operation to the action list.
    pub fn set_value_op(&mut self, value: U256) -> &mut Self {
        self.actions.push(Action::SetValue { value });
        self
    }

    /// Adds an `EXTCODECOPY` operation to the action list.
    pub fn set_extcodecopy_op(
        &mut self,
        source: Address,
        data_offset: u16,
        code_offset: u16,
        size: u16,
    ) -> &mut Self {
        self.actions.push(Action::ExtCodeCopy {
            source,
            data_offset,
            code_offset,
            size,
        });
        self
    }

    /// Adds a `CALL` operation to the action list.
    pub fn call_op(&mut self) -> &mut Self {
        self.actions.push(Action::Call);
        self
    }

    /// Adds a `CREATE` operation to the action list.
    ///
    /// `created_address` is the expected address of the deployed contract,
    /// used by the peephole optimizer to eliminate redundant `SETADDR` ops.
    pub fn create_op(&mut self, created_address: Address) -> &mut Self {
        self.actions.push(Action::Create { created_address });
        self
    }

    /// Adds a `DELEGATECALL` operation to the action list.
    pub fn delegatecall_op(&mut self) -> &mut Self {
        self.actions.push(Action::DelegateCall);
        self
    }

    // -- High-level helpers ------------------------------------------------

    /// Prepares a `CALL`: sets target, value, data buffer, then executes.
    pub fn call(&mut self, target: Address, data: &[u8], value: U256) -> &mut Self {
        assert!(
            data.len() <= u16::MAX as usize,
            "data length exceeds u16::MAX"
        );
        self.set_addr_op(target)
            .set_value_op(value)
            .set_cleardata_op(data.len() as u16)
            .set_data_op(0, data)
            .call_op()
    }

    /// Prepares a `DELEGATECALL`: sets target, data buffer, then executes.
    pub fn delegatecall(&mut self, target: Address, data: &[u8]) -> &mut Self {
        self.set_addr_op(target)
            .set_cleardata_op(data.len() as u16)
            .set_data_op(0, data)
            .delegatecall_op()
    }

    /// Prepares a `CREATE`: sets value, data buffer, then deploys.
    pub fn create(&mut self, created_address: Address, data: &[u8], value: U256) -> &mut Self {
        self.set_value_op(value)
            .set_cleardata_op(data.len() as u16)
            .set_data_op(0, data)
            .create_op(created_address)
    }

    /// Sets the callback address for flash loan handlers.
    pub fn set_callback(&mut self, callback_address: Address) -> &mut Self {
        self.actions.push(Action::SetCallback { callback_address });
        self
    }

    /// Marks subsequent calls as must-succeed (revert on failure).
    pub fn set_fail(&mut self) -> &mut Self {
        self.actions.push(Action::SetFail);
        self
    }

    /// Clears the must-succeed flag.
    pub fn clear_fail(&mut self) -> &mut Self {
        self.actions.push(Action::ClearFail);
        self
    }

    // -- Optimizer ---------------------------------------------------------

    /// Runs the peephole optimizer to remove redundant operations.
    pub fn optimize(&mut self) -> &mut Self {
        self.peephole_opt();
        self
    }

    /// Single-pass peephole optimizer.  Marks redundant actions, then removes
    /// them via `retain` in O(n).
    fn peephole_opt(&mut self) {
        let mut last_value = U256::ZERO;
        let mut last_target = Address::ZERO;
        let mut last_data: Vec<u8> = Vec::new();
        let mut last_fail = false;

        let mut keep = vec![true; self.actions.len()];

        for (idx, action) in self.actions.iter().enumerate() {
            let redundant = match action {
                Action::SetFail => {
                    if last_fail {
                        true
                    } else {
                        last_fail = true;
                        false
                    }
                }
                Action::ClearFail => {
                    if last_fail {
                        last_fail = false;
                        false
                    } else {
                        true
                    }
                }
                Action::Call => {
                    last_value = U256::ZERO;
                    false
                }
                Action::Create { created_address } => {
                    last_target = *created_address;
                    last_value = U256::ZERO;
                    false
                }
                Action::SetAddr { addr } => {
                    let res = last_target == *addr;
                    last_target = *addr;
                    res
                }
                Action::SetValue { value } => {
                    let res = last_value == *value;
                    last_value = *value;
                    res
                }
                Action::ClearData { size } => {
                    let res = last_data.len() == *size as usize;
                    last_data = vec![0; *size as usize];
                    res
                }
                Action::SetData { offset, data } => {
                    let offset_uz = *offset as usize;
                    let mut new_data = last_data.clone();
                    new_data.splice(offset_uz..offset_uz + data.len(), data.to_owned());
                    let res = last_data == new_data;
                    last_data = new_data;
                    res
                }
                _ => false,
            };
            if redundant {
                keep[idx] = false;
            }
        }

        let mut i = 0;
        self.actions.retain(|_| {
            let k = keep[i];
            i += 1;
            k
        });
    }

    // -- Encoding ----------------------------------------------------------

    /// Encodes the action list into raw bytecode (no function selector).
    pub fn build_raw(&self) -> Vec<u8> {
        self.actions.iter().flat_map(|a| a.encode()).collect()
    }

    /// Encodes the action list into calldata for `executeActions()`.
    pub fn build(&self) -> Vec<u8> {
        let mut res = EXECUTE_ACTIONS_SELECTOR.to_vec();
        res.extend(self.build_raw());
        res
    }
}
