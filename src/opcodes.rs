/// Operation opcodes for the executor bytecode format.
///
/// Each constant represents a single-byte opcode consumed by the
/// on-chain `executor` contract's `_executeActions` interpreter loop.

pub const OP_EOF: u8 = 0x00;
pub const OP_CLEARDATA: u8 = 0x01;
pub const OP_SETDATA: u8 = 0x02;
pub const OP_SETADDR: u8 = 0x03;
pub const OP_SETVALUE: u8 = 0x04;
pub const OP_EXTCODECOPY: u8 = 0x05;
pub const OP_CALL: u8 = 0x06;
pub const OP_CREATE: u8 = 0x07;
pub const OP_DELEGATECALL: u8 = 0x08;
pub const OP_SETCALLBACK: u8 = 0x09;
pub const OP_SETFAIL: u8 = 0x0a;
pub const OP_CLEARFAIL: u8 = 0x0b;
