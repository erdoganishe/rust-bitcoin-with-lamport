mod key_generation;
mod lamport;
mod script_generation;
mod own_lamport_sig;

use bdk::bitcoin::secp256k1::Message;
use key_generation::{generate_and_write_key_pairs, read_keys_from_files, sign_message, verify_signature};
use own_lamport_sig::{signature, write_keys_to_file};
use script_generation::build_script;
//use lamport::{generate_keys, sign_message, verify_signature};
fn main() {
    //let _ = write_keys_to_file(1);
    let sig = signature(false);
    let res = own_lamport_sig::verify_signature(false, sig.clone());
    println!("{:#?}, {}", sig, res);
}
