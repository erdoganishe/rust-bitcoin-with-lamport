mod key_generation;
mod lamport;
mod script_generation;

use bdk::bitcoin::secp256k1::Message;
use key_generation::{generate_and_write_key_pairs, read_keys_from_files, sign_message, verify_signature};
use script_generation::build_script;
//use lamport::{generate_keys, sign_message, verify_signature};
fn main() {
    //new keys generation

    // generate_and_write_key_pairs(1);

    // //check keys for valid

    // // let prv1: Vec<String> = vec!("0130b0cddc89db7d13954956fe216bcf7881582e539bdffcaf31053f5459bc8f".to_string());
    // // let pub1: Vec<String> = vec!("2d9f378b804c76e7c1fbf661e4d31fe8707bfba871f6baa239e8351fd35b7c22".to_string());
    // // println!("{}", verify_encoded_key_pair(&prv1, &pub1));
    
    // //read keys from file

    // let (private_keys, public_keys ) = read_keys_from_files("private_keys.txt", "public_keys.txt");


    // let message = "123";
    // let signature = sign_message(&message, &private_keys);
    // let verify  = verify_signature(&message, &signature, &public_keys);
    // println!("msg: {}, sig {:#?}, verified {} ", message, signature, verify);

   

    let script = build_script();
    println!("Generated Script: {}", script);


}
