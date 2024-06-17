use bitcoin::blockdata::script::{Builder, Script};
use bitcoin::blockdata::opcodes;
use bitcoin::ScriptBuf;
use bitcoin::opcodes::OP_0;

pub fn build_script() -> ScriptBuf {
    let mut builder = Builder::new();

    builder = builder.push_opcode(opcodes::all::OP_CHECKSIGVERIFY); // OP_CHECKSIGVERIFY
    builder = builder.push_opcode(OP_0); // OP_0
    builder = builder.push_opcode(opcodes::all::OP_SWAP); // OP_SWAP
    builder = builder.push_opcode(opcodes::all::OP_DUP); // OP_DUP

    // Push 32-byte sequence
    let image_0 = [
        0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
        0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e,
        0x3f, 0x40,
    ];
    builder = builder.push_slice(&image_0); // OP_PUSHBYTES_32 <data>

    builder = builder.push_opcode(opcodes::all::OP_EQUAL); // OP_EQUAL
    builder = builder.push_opcode(opcodes::all::OP_IF); // OP_IF
    builder = builder.push_opcode(opcodes::all::OP_DROP); // OP_DROP
    builder = builder.push_int(1); // OP_1 (assuming this is what you intended for OP_PUSHNUM_1)
    builder = builder.push_opcode(opcodes::all::OP_ELSE); // OP_ELSE

    // Push 32-byte sequence
    let image_1 = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];
    builder = builder.push_slice(&image_1); // OP_PUSHBYTES_32 <data>

    builder = builder.push_opcode(opcodes::all::OP_EQUALVERIFY); // OP_EQUALVERIFY
    builder = builder.push_opcode(opcodes::all::OP_ENDIF); // OP_ENDIF

    builder.into_script()
}

// fn main() {
//     let script = build_script();
//     println!("Built script: {:?}", script);
// }
