use alloy_primitives::{Address, U256};

use crate::opcodes::{Call, ClearData, Create, DelegateCall, ExtCodeCopy, SetAddr, SetData, SetValue};

// Enum for all opcode actions
enum Action {
    ClearData(ClearData),
    SetData(SetData),
    SetAddr(SetAddr),
    SetValue(SetValue),
    ExtCodeCopy(ExtCodeCopy),
    Call(Call),
    Create(Create),
    DelegateCall(DelegateCall),
}

impl Action {
    fn encode(&self) -> Vec<u8> {
        match self {
            Action::ClearData(cd) => cd.encode(),
            Action::SetData(sd) => sd.encode(),
            Action::SetAddr(sa) => sa.encode(),
            Action::SetValue(sv) => sv.encode(),
            Action::ExtCodeCopy(ecc) => ecc.encode(),
            Action::Call(c) => c.encode(),
            Action::Create(c) => c.encode(),
            Action::DelegateCall(dc) => dc.encode(),
        }
    }
}

// FlowBuilder to manage the actions
#[derive(Default)]
pub struct FlowBuilder {
    actions: Vec<Action>,
}

impl FlowBuilder {
    /// Creates an empty `FlowBuilder` with no actions.
    pub fn empty() -> Self {
        Self::default()
    }

    /// A simple optimizer that will remove redundant sets
    fn peephole_opt(&mut self) {
        let mut ops_to_remove = Vec::new();
        let mut last_value = U256::ZERO;
        let mut last_target = Address::ZERO;
        let mut last_data: Vec<u8> = Vec::new();

        for (idx, action) in self.actions.iter().enumerate() {
            let to_remove = match action {
                Action::Call(_) => {
                    last_value = U256::ZERO;
                    false
                },
                Action::Create(Create { created_address }) => {
                    last_target = *created_address;
                    last_value = U256::ZERO;
                    false
                },
                Action::SetAddr(SetAddr { addr }) => {
                    let res = last_target == *addr;
                    last_target = *addr;
                    res
                },
                Action::SetValue(SetValue { value }) => {
                    let res = last_value == *value;
                    last_value = *value;
                    res
                },
                Action::ClearData(ClearData { size }) => {
                    let res = last_data.len() == *size as usize;
                    last_data = vec![0; *size as usize];
                    res
                },
                Action::SetData(SetData { offset, data }) => {
                    let offset_uz = *offset as usize;
                    let mut new_data = last_data.clone();
                    new_data.splice(offset_uz  .. offset_uz + data.len(), data.to_owned());
                    let res = last_data == new_data;
                    last_data = new_data;
                    res
                },
                _ => false,
            };
            if to_remove {
                ops_to_remove.push(idx);
            }
        }

        for idx in ops_to_remove.into_iter().rev() {
            self.actions.remove(idx);
        }
    }

    /// Adds an `EXTCODECOPY` operation to the action list.
    pub fn set_extcodecopy_op(mut self, source: Address, data_offset: u16, code_offset: u16, size: u16) -> Self {
        self.actions.push(Action::ExtCodeCopy(ExtCodeCopy{ source, data_offset, code_offset, size }));
        self
    }

    /// Adds a `SETADDR` operation to the action list.
    pub fn set_addr_op(mut self, addr: Address) -> Self {
        self.actions.push(Action::SetAddr(SetAddr { addr }));
        self
    }

    /// Adds a `SETVALUE` operation to the action list.
    pub fn set_value_op(mut self, value: U256) -> Self {
        self.actions.push(Action::SetValue(SetValue { value }));
        self
    }

    /// Adds a `SETDATA` operation to the action list.
    pub fn set_data_op(mut self, offset: u16, data: &[u8]) -> Self {
        self.actions.push(Action::SetData(SetData {
            offset,
            data: data.to_owned(),
        }));
        self
    }

    /// Adds a `CLEARDATA` operation to the action list.
    pub fn set_cleardata_op(mut self, size: u16) -> Self {
        self.actions.push(Action::ClearData(ClearData { size }));
        self
    }

    /// Adds a `CALL` operation to the action list.
    pub fn call_op(mut self) -> Self {
        self.actions.push(Action::Call(Call::new()));
        self
    }

    /// Adds a `CREATE` operation to the action list.
    pub fn create_op(mut self, created_address: Address) -> Self {
        self.actions
            .push(Action::Create(Create { created_address }));
        self
    }

    /// Adds a `DELEGATECALL` operation to the action list.
    pub fn delegatecall_op(mut self) -> Self {
        self.actions.push(Action::DelegateCall(DelegateCall::new()));
        self
    }

    /// Prepares a `CALL` operation with the specified target, data, and value.
    pub fn call(self, target: Address, data: &[u8], value: U256) -> Self {
        assert!(data.len() < u16::MAX as usize, "datalen exceeds 0xffff");

        self.set_addr_op(target)
            .set_value_op(value)
            .set_cleardata_op(data.len() as u16)
            .set_data_op(0, data)
            .call_op()
    }

    /// Prepares a `DELEGATECALL` operation with the specified target and data.
    pub fn delegatecall(self, target: Address, data: &[u8]) -> Self {
        self.set_addr_op(target)
            .set_cleardata_op(data.len() as u16)
            .set_data_op(0, data)
            .delegatecall_op()
    }

    /// Prepares a `CREATE` operation with the specified address, data, and value.
    pub fn create(self, created_address: Address, data: &[u8], value: U256) -> Self {
        self.set_value_op(value)
            .set_cleardata_op(data.len() as u16)
            .set_data_op(0, data)
            .create_op(created_address)
    }

    /// Builds the sequence of operations into a byte vector, optionally optimizing it.
    pub fn build(mut self, enable_opt: bool) -> Vec<u8> {
        let mut res = Vec::new();
        if enable_opt {
            self.peephole_opt();
        }
        for action in self.actions {
            res.extend(&action.encode());
        }
        res
    }
}
