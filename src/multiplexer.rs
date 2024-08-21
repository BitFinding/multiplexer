use alloy::primitives::{Address, U256};

// Operation opcodes as constants
pub const OP_CLEARDATA: u8 = 0x01;
pub const OP_SETDATA: u8 = 0x01;
pub const OP_SETADDR: u8 = 0x02;
pub const OP_SETVALUE: u8 = 0x03;
pub const OP_EXTCODECOPY: u8 = 0x04;
pub const OP_CALL: u8 = 0x05;
pub const OP_CREATE: u8 = 0x06;
pub const OP_DELEGATECALL: u8 = 0x07;

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
        let mut encoded = Vec::new();
        encoded.push(OP_SETDATA); // Opcode
        encoded.extend(&self.offset.to_be_bytes()); // Offset
        encoded.extend(&self.data); // Data
        encoded
    }
}

// Struct for the RESETDATA operation
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

// Struct for the SETTARGET operation
pub struct SetAddr {
    pub target: Address, // 20-byte address
}

impl SetAddr {
    pub fn new(target: Address) -> Self {
        SetAddr { target }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_SETADDR); // Opcode
        encoded.extend(&self.target); // Target address
        encoded
    }
}

// Struct for the CALL operation
#[derive(Default)]
pub struct Call;

impl Call {
    pub fn new() -> Self {
        Self {}
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

#[derive(Clone, Debug)]
struct SetValue {
    pub value: U256,
}

impl SetValue {
    pub fn new(value: U256) -> Self {
        SetValue { value }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        // Add SetValue encoding logic if applicable
        encoded
    }
}

enum Action {
    Call(Call),
    SetData(SetData),
    SetTarget(SetAddr),
    SetValue(SetValue),
    ClearData(ClearData),
    Create(Create),
}

impl Action {
    fn encode(&self) -> Vec<u8> {
        match self {
            Action::Call(c) => c.encode(),
            Action::SetData(sd) => sd.encode(),
            Action::SetTarget(st) => st.encode(),
            Action::SetValue(sv) => sv.encode(),
            Action::ClearData(cd) => cd.encode(),
            Action::Create(c) => c.encode(),
        }
    }
}

#[derive(Default)]
pub struct FlowBuilder {
    actions: Vec<Action>,
}

impl FlowBuilder {
    fn empty() -> Self {
        Self::default()
    }

    fn set_target_op(&mut self, target: Address) {
        self.actions.push(Action::SetTarget(SetAddr { target }));
    }

    fn set_value_op(&mut self, value: U256) {
        self.actions.push(Action::SetValue(SetValue { value }));
    }

    fn set_data_op(&mut self, offset: u16, data: &[u8]) {
        self.actions.push(Action::SetData(SetData {
            offset,
            data: data.to_owned(),
        }))
    }

    fn set_cleardata_op(&mut self, size: u16) {
        self.actions.push(Action::ClearData(ClearData { size }));
    }

    fn call_op(&mut self) {
        self.actions.push(Action::Call(Call {}))
    }

    fn create_op(&mut self, created_address: Address) {
        self.actions
            .push(Action::Create(Create { created_address }))
    }

    pub fn call(mut self, target: Address, data: &[u8], value: U256) -> Self {
        self.set_target_op(target);
        self.set_value_op(value);
        self.set_cleardata_op(data.len() as u16);
        self.set_data_op(0, data);
        self.call_op();
        self
    }

    pub fn create(mut self, created_address: Address, data: &[u8], value: U256) -> Self {
        self.set_value_op(value);
        self.set_cleardata_op(data.len() as u16);
        self.set_data_op(0, data);
        self.create_op(created_address);
        self
    }

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
                }
                Action::Create(Create { created_address }) => {
                    last_target = *created_address;
                    last_value = U256::ZERO;
                    false
                }
                Action::SetTarget(SetAddr { target }) => {
                    let res = last_target == *target;
                    last_target = *target;
                    res
                }
                Action::SetValue(SetValue { value }) => {
                    let res = last_value == *value;
                    last_value = *value;
                    res
                }
                Action::ClearData(ClearData { size }) => {
                    let res = last_data.len() == *size as usize;
                    last_data = vec![0; *size as usize];
                    res
                }
                Action::SetData(SetData { offset, data }) => {
                    let offset_uz = *offset as usize;
                    let mut new_data = last_data.clone();
                    new_data.splice(offset_uz..offset_uz + data.len(), data.to_owned());
                    let res = last_data == new_data;
                    last_data = new_data;
                    res
                }
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

    pub fn build(mut self) -> Vec<u8> {
        self.peephole_opt();
        let mut res = Vec::new();
        for action in self.actions {
            res.extend(&action.encode())
        }
        res
    }
}

#[test]
fn test() {
    let calldata = FlowBuilder::empty()
        .create(Address::ZERO, &Vec::new(), U256::from(1))
        .call(Address::ZERO, &vec![0, 1], U256::ZERO)
        .build();
    println!("Encoded calldata {:?}", calldata);

    // Example: Create a SETDATA operation
    let setdata_op = SetData::new(16, b"Some Data".to_vec());
    let encoded_setdata = setdata_op.encode();
    println!("Encoded SETDATA operation: {:?}", encoded_setdata);

    // Example: Create a CALL operation
    let call_op = Call::new();
    let encoded_call = call_op.encode();
    println!("Encoded CALL operation: {:?}", encoded_call);
}
