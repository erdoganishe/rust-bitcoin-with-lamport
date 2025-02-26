mod script_generation;
mod own_lamport_sig;
mod tx_creation;
mod tx;
mod test;
mod bip_test;

use own_lamport_sig::{signature, write_keys_to_file};
use script_generation::build_script;
use test::gen_input_tx_test;
use tx::gen_input_tx;
use tx_creation::get_address_from_public_key;
// use tx::{gen_input_tx, psbt_creation};
// use tx::psbt_creation;
// use tx_creation::get_descriptor;
// use tx_creation::tx_creation;
//use lamport::{generate_keys, sign_message, verify_signature};
fn main() {
    
    let _ = write_keys_to_file(4, "private_long_keys.txt","public_long_keys.txt");
    // let sig = long_signature(vec!(false,false,true,false));
    // let res = verify_long_signature(vec!(false,false,true,false), sig.clone());
    // println!("{:#?}, {}", sig, res);
    let message = true;
    let sig = signature(message);

    let res_script = build_script(
        sig.clone(), 
        message,
        vec!(
            "abf33e29af0d2e11f69bbe7c5c2aa6b6009747e6a0590e9e776d3fc2ddffd3fc".to_string(),
            "b67bcaefdab1c1f95e5274d103ca0203819fc80d0b4231a5aec85b813de853e1".to_string()
        )
    );

    // println!("{}", res_script);
    println!("{}", res_script.to_p2sh());
    let s =  res_script.to_hex_string();
    println!("Address: {}", get_address_from_public_key());

    let script: bdk::bitcoin::ScriptBuf = bdk::bitcoin::ScriptBuf::from_hex(&s).unwrap();
    println!("{}", script.to_p2sh());
    gen_input_tx(script);
    // psbt_creation(res_script, sig);
    // println!("{}", res_script.to_p2sh());


    //get_descriptor(res_script.clone().to_p2sh());

    // tx_creation::tx_creation();
    // println!("{}", res);


    // let tx = tx_creation( sig, 
    //     message,
    //     vec!(
    //         "abf33e29af0d2e11f69bbe7c5c2aa6b6009747e6a0590e9e776d3fc2ddffd3fc".to_string(),
    //         "b67bcaefdab1c1f95e5274d103ca0203819fc80d0b4231a5aec85b813de853e1".to_string()
    //     ));

    // println!("{:#?}", tx.unwrap());

    gen_input_tx_test();

}
