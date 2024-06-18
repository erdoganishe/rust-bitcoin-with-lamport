use bdk::bitcoin::ScriptBuf;
use bdk::bitcoin::{opcodes::all::{OP_CHECKSIGVERIFY, OP_ELSE, OP_ENDIF, OP_EQUAL, OP_IF, OP_SHA256}, script::Builder};



pub fn build_script(sig: Vec<String>, message: bool, pub_keys: Vec<String>) -> ScriptBuf{

    let message_int:i64 = match message{
        true=>1,
        false=>0
    };

    // 0 0 OP_EQUAL op_if
    // abf33e29af0d2e11f69bbe7c5c2aa6b6009747e6a0590e9e776d3fc2ddffd3fc 
    // OP_else
    // b67bcaefdab1c1f95e5274d103ca0203819fc80d0b4231a5aec85b813de853e1 
    // op_endif
    // 26eee2d8c8c8e14066a4eff6c3f6e451c95389dda2526a048c5e4fbb0d53df1a 
    // op_sha256 
    // op_checksigverify


    let script_builer: ScriptBuf = Builder::new()
        .push_int(message_int)
        .push_int(0)
        .push_opcode(OP_EQUAL)
        .push_opcode(OP_IF)
        // .push_...pubkey[0]
        .push_opcode(OP_ELSE)
        // .push_...pubkey[1]
        .push_opcode(OP_ENDIF)
        // .push_...sig[0]
        .push_opcode(OP_SHA256)
        .push_opcode(OP_CHECKSIGVERIFY)
        .into_script();

    script_builer
}