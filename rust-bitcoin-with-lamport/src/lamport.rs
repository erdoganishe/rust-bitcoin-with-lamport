// use bitcoin::blockdata::script::{Builder, Script};
// use bitcoin::hashes::sha256;
// use bitcoin::opcodes::OP_0;
// use bitcoin::ScriptBuf;
// use sha2::{Digest, Sha256};
// use bitcoin_scripts::ScriptCode;



// fn build_script(pubkey: &[u8; 33], keys: &[(Vec<u8>, Vec<u8>)]) ->ScriptCode  {
//     let mut builder = Builder::new();

//     builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKSIGVERIFY);

//     // Initialize accumulator with 0
//     builder = builder.push_opcode(OP_0);

//     // Build the main body of the script
//     for (idx, (image_0, image_1)) in keys.iter().enumerate() {
//         // SWAP operation (not directly available in Bitcoin Script)
//         builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_SWAP);

//         // SHA256 of witness data
//         let witness_sha256 = Sha256::digest(pubkey);

//         // Conditional branch based on SHA256 results
//         builder = builder
//             .push_opcode(bitcoin::blockdata::opcodes::all::OP_SHA256)
//             .push_slice(&witness_sha256);

//         // DUP operation (duplicate top stack item)
//         builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_DUP);

//         // Compare with <H(K_i_1)>
//         builder = builder.push_slice(&image_1);
//         builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_EQUAL);
//         builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_IF);
//         builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_DROP);
//         builder = builder.push_int(((1 as u32) << idx) as i64);
//         builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_ELSE);

//         // Compare with <H(K_i_0)>
//         builder = builder.push_slice(&image_0);
//         builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_EQUALVERIFY);
//         builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_ENDIF);
//     }

//     // Add CHECKSEQUENCEVERIFY operation (optional, if needed)
//     builder = builder.push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKSEQUENCEVERIFY);

//     // Return the built script
//     builder.into_script()
// }
