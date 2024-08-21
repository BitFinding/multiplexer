use alloy::primitives::{Address, U256};

// Operation opcodes as constants
pub const OP_CLEARDATA: u8 = 0x00;
pub const OP_SETDATA: u8 = 0x01;
pub const OP_SETADDR: u8 = 0x02;
pub const OP_SETVALUE: u8 = 0x03;
pub const OP_EXTCODECOPY: u8 = 0x04;
pub const OP_CALL: u8 = 0x05;
pub const OP_CREATE: u8 = 0x06;
pub const OP_DELEGATECALL: u8 = 0x07;

// Struct for the CLEARDATA operation
pub struct ClearData {
    pub size: u16,
}

impl ClearData {
    pub fn new(size: u16) -> Self {
        ClearData { size }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_CLEARDATA); // Opcode
        encoded.extend(&self.size.to_be_bytes()); // Size
        encoded
    }
}

// Struct for the SETDATA operation
pub struct SetData {
    pub offset: u16,
    pub data: Vec<u8>,
}

impl SetData {
    pub fn new(offset: u16, data: Vec<u8>) -> Self {
        SetData { offset, data }
    }

    pub fn encode(&self) -> Vec<u8> {
        let data_size = self.data.len() as u16;
        let mut encoded = Vec::new();
        encoded.push(OP_SETDATA); // Opcode
        encoded.extend(&self.offset.to_be_bytes()); // Offset
        encoded.extend(&data_size.to_be_bytes()); // Data Size
        encoded.extend(&self.data); // Data
        encoded
    }
}

// Struct for the SETADDR operation
pub struct SetAddr {
    pub addr: Address, // 20-byte address
}

impl SetAddr {
    pub fn new(addr: Address) -> Self {
        SetAddr { addr }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_SETADDR); // Opcode
        encoded.extend(&self.addr); // Address
        encoded
    }
}

// Struct for the SETVALUE operation
#[derive(Clone, Debug)]
pub struct SetValue {
    pub value: U256,
}

impl SetValue {
    pub fn new(value: U256) -> Self {
        SetValue { value }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_SETVALUE); // Opcode
        encoded.extend(&self.value.to_be_bytes::<32>()); // Value
        encoded
    }
}

// Struct for the EXTCODECOPY operation
pub struct ExtCodeCopy {
    pub source: Address, // Address of contract to copy code from
    pub offset: u16,     // Offset to copy code to
    pub size: u16,       // Size of the code to copy
}

impl ExtCodeCopy {
    pub fn new(source: Address, offset: u16, size: u16) -> Self {
        ExtCodeCopy {
            source,
            offset,
            size,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_EXTCODECOPY); // Opcode
        encoded.extend(&self.source); // Source address
        encoded.extend(&self.offset.to_be_bytes()); // Offset
        encoded.extend(&self.size.to_be_bytes()); // Size
        encoded
    }
}

// Struct for the CALL operation
#[derive(Default)]
pub struct Call {}

impl Call {
    pub fn new() -> Self {
        Call {}
    }

    pub fn encode(&self) -> Vec<u8> {
        vec![OP_CALL] // Opcode
    }
}

// Struct for the CREATE operation
#[derive(Default)]
pub struct Create {
    pub created_address: Address,
}

impl Create {
    pub fn new(created_address: Address) -> Self {
        Self { created_address }
    }

    pub fn encode(&self) -> Vec<u8> {
        vec![OP_CREATE] // Opcode
    }
}

// Struct for the DELEGATECALL operation
#[derive(Default)]
pub struct DelegateCall {}

impl DelegateCall {
    pub fn new() -> Self {
        DelegateCall {}
    }

    pub fn encode(&self) -> Vec<u8> {
        vec![OP_DELEGATECALL] // Opcode
    }
}

// Enum for all actions
pub enum Action {
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
    fn empty() -> Self {
        Self::default()
    }

    fn set_addr_op(&mut self, addr: Address) {
        self.actions.push(Action::SetAddr(SetAddr { addr }));
    }

    fn set_value_op(&mut self, value: U256) {
        self.actions.push(Action::SetValue(SetValue { value }));
    }

    fn set_data_op(&mut self, offset: u16, data: &[u8]) {
        self.actions.push(Action::SetData(SetData {
            offset,
            data: data.to_owned(),
        }));
    }

    fn set_cleardata_op(&mut self, size: u16) {
        self.actions.push(Action::ClearData(ClearData { size }));
    }

    fn call_op(&mut self) {
        self.actions.push(Action::Call(Call::new()));
    }

    fn create_op(&mut self, created_address: Address) {
        self.actions
            .push(Action::Create(Create { created_address }));
    }

    fn delegatecall_op(&mut self) {
        self.actions.push(Action::DelegateCall(DelegateCall::new()));
    }

    pub fn call(mut self, target: Address, data: &[u8], value: U256) -> Self {
        assert!(data.len() < u16::MAX as usize, "datalen exceeds 0xffff");

        self.set_addr_op(target);
        self.set_value_op(value);
        self.set_cleardata_op(data.len() as u16);
        self.set_data_op(0, data);
        self.call_op();
        self
    }

    pub fn delegatecall(mut self, target: Address, data: &[u8]) -> Self {
        self.set_addr_op(target);
        self.set_cleardata_op(data.len() as u16);
        self.set_data_op(0, data);
        self.delegatecall_op();
        self
    }

    pub fn create(mut self, created_address: Address, data: &[u8], value: U256) -> Self {
        self.set_value_op(value);
        self.set_cleardata_op(data.len() as u16);
        self.set_data_op(0, data);
        self.create_op(created_address);
        self
    }

    pub fn build(mut self) -> Vec<u8> {
        let mut res = Vec::new();
        for action in self.actions {
            res.extend(&action.encode());
        }
        res
    }
}

#[test]
fn test() {
    let addr_a = Address::repeat_byte(0x41);
    let addr_b = Address::repeat_byte(0x42);
    let calldata = FlowBuilder::empty()
        .create(Address::ZERO, "LALA".as_bytes(), U256::from(10))
        .call(addr_a, &vec![98, 99], U256::ZERO)
        .delegatecall(addr_b, &vec![70, 71])
        .build();
    println!("Encoded calldata {:?}", calldata);
}
