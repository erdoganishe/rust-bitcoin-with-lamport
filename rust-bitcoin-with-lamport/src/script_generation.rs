use std::str::FromStr;

use bdk::bitcoin::ScriptBuf;
use bdk::bitcoin::{opcodes::all::{OP_CHECKSIGVERIFY, OP_ELSE, OP_ENDIF, OP_EQUAL, OP_IF, OP_SHA256}, script::Builder};
use bdk::miniscript::serde::Serialize;
use bitcoin::script::PushBytes;
use bitcoin::PublicKey;
use hex::decode;


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

    // let pubkey0_bytes = PublicKey::from_str(&pub_keys[0].clone()).unwrap();
    // let pubkey1_bytes = PublicKey::from_str(&pub_keys[1].clone()).unwrap();
    // let sig_bytes = PublicKey::from_str(&sig[0].clone()).unwrap();
    let pubkey0_bytes = decode(pub_keys[0].clone()).expect("Error with decoding public key");
    let mut pubkey0_array: [u8; 32] = [0; 32];
    if pubkey0_bytes.len() == 32 {
        pubkey0_array.copy_from_slice(&pubkey0_bytes);
    } else {
        panic!("pubkey0_bytes does not have exactly 32 bytes");
    }

    let pubkey1_bytes = decode(pub_keys[1].clone()).expect("Error with decoding public key");
    let mut pubkey1_array: [u8; 32] = [0; 32];
    if pubkey1_bytes.len() == 32 {
        pubkey1_array.copy_from_slice(&pubkey1_bytes);
    } else {
        panic!("pubkey0_bytes does not have exactly 32 bytes");
    }
    
    let sig_bytes = decode(sig[0].clone()).expect("Error with decoding public key");
    let mut sig_array: [u8; 32] = [0; 32];
    if sig_bytes.len() == 32 {
        sig_array.copy_from_slice(&sig_bytes);
    } else {
        panic!("pubkey0_bytes does not have exactly 32 bytes");
    }

    // let pubkey1_bytes = hex::decode(pub_keys[1].clone()).expect("Error with decoding public key");
    // let sig_bytes: &PushBytes =  <&PushBytes>::try_from(sig[0].clone().as_bytes()).unwrap();

    //script sig / witness
    let script_builer: ScriptBuf = Builder::new()
        .push_int(message_int)
        .push_int(0)
        .push_opcode(OP_EQUAL)
        .push_opcode(OP_IF)
        .push_slice(&pubkey0_array)
        .push_opcode(OP_ELSE)
        .push_slice(&pubkey1_array)
        .push_opcode(OP_ENDIF)
        .push_slice(&sig_array)
        .push_opcode(OP_SHA256)
        .push_opcode(OP_CHECKSIGVERIFY)
        .into_script();

    script_builer
}