mod key_generation;
mod lamport;
mod script_generation;
mod own_lamport_sig;

use bdk::bitcoin::secp256k1::Message;
use key_generation::{generate_and_write_key_pairs, read_keys_from_files, sign_message, verify_signature};
use own_lamport_sig::{long_signature, signature, verify_long_signature, write_keys_to_file};
use script_generation::build_script;
//use lamport::{generate_keys, sign_message, verify_signature};
fn main() {
    let _ = write_keys_to_file(4, "private_long_keys.txt","public_long_keys.txt");
    let sig = long_signature(vec!(false,false,true,false));
    let res = verify_long_signature(vec!(false,false,true,false), sig.clone());
    println!("{:#?}, {}", sig, res);
}
