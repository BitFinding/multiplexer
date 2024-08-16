use alloy::primitives::Address;

// Operation opcodes as constants
pub const OP_SETDATA: u8 = 0x01;
pub const OP_RESETDATA: u8 = 0x02;
pub const OP_CALL: u8 = 0x03;
pub const OP_CREATE: u8 = 0x04;
pub const OP_SETTARGET: u8 = 0x05;
pub const OP_SETALLOWFAIL: u8 = 0x06;
pub const OP_PATCH: u8 = 0x07;
pub const OP_EXTCODECOPY: u8 = 0x08;

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
        encoded.push(OP_SETDATA);                // Opcode
        encoded.extend(&self.offset.to_be_bytes()); // Offset
        encoded.extend(&data_size.to_be_bytes());  // Data
        encoded.extend(&self.data);  // Data
        encoded
    }
}

// Struct for the RESETDATA operation
pub struct ResetData {
    pub size: u16,
}

impl ResetData {
    pub fn new(size: u16) -> Self {
        ResetData { size }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_RESETDATA);              // Opcode
        encoded.extend(&self.size.to_be_bytes());   // Size
        encoded
    }
}

// Struct for the SETTARGET operation
pub struct SetTarget {
    pub target: Address,  // 20-byte address
}

impl SetTarget {
    pub fn new(target: Address) -> Self {
        SetTarget { target }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_SETTARGET);              // Opcode
        encoded.extend(&self.target);            // Target address
        encoded
    }
}

// Struct for the SETALLOWFAIL operation
pub struct SetAllowFail {
    pub allow_fail: bool,
}

impl SetAllowFail {
    pub fn new(allow_fail: bool) -> Self {
        SetAllowFail { allow_fail }
    }

    pub fn encode(&self) -> Vec<u8> {
        vec![OP_SETALLOWFAIL,                           // Opcode
            if self.allow_fail { 0x01 } else { 0x00 }   // Allow fail
            ]
    }
}

// Struct for the CALL operation
#[derive(Default)]
pub struct Call {
}

impl Call {
    pub fn new() -> Self { Self {} }

    pub fn encode(&self) -> Vec<u8> {
        vec![OP_CALL] // Opcode
    }
}

// Struct for the CREATE operation
#[derive(Default)]
pub struct Create {
}
impl Create {
    pub fn new() -> Self { Self {} }

    pub fn encode(&self) -> Vec<u8> {
        vec![OP_CREATE] // Opcode
    }
}

// Struct for the PATCH operation
pub struct Patch {
    pub patches: Vec<(u16, Vec<u8>)>,  // Offset and data patches
}

impl Patch {
    pub fn new(patches: Vec<(u16, Vec<u8>)>) -> Self {
        Patch { patches }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.push(OP_PATCH);                  // Opcode
        for (offset, data) in &self.patches {
            let data_size = data.len() as u16;  // Len should fit in 64k
            encoded.extend(&offset.to_be_bytes()); // Patch offset
            encoded.extend(&data_size.to_be_bytes()); // Patch offset
            encoded.extend(data);                  // Patch data
        }
        encoded
    }
}

// Struct for the EXTCODECOPY operation
pub struct ExtCodeCopy {
    pub source: Address,  // Address of contract to copy code from
    pub offset: u16,       // Offset to copy code to
    pub size: u16,         // Size of the code to copy
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
        encoded.push(OP_EXTCODECOPY);            // Opcode
        encoded.extend(&self.source);            // Source address
        encoded.extend(&self.offset.to_be_bytes()); // Offset
        encoded.extend(&self.size.to_be_bytes());   // Size
        encoded
    }
}

fn main() {
    // Example: Create a SETDATA operation
    let setdata_op = SetData::new(16, b"Some Data".to_vec());
    let encoded_setdata = setdata_op.encode();
    println!("Encoded SETDATA operation: {:?}", encoded_setdata);

    // Example: Create a CALL operation
    let call_op = Call::new(); // Value: 1,000,000 Wei
    let encoded_call = call_op.encode();
    println!("Encoded CALL operation: {:?}", encoded_call);
}