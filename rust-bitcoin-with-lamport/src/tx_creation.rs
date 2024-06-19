// use bdk::bitcoin::network::address;
// use bdk::bitcoin::{Address, Network, PublicKey, ScriptBuf};
// use bdk::database::MemoryDatabase;
// use bdk::miniscript::descriptor::Descriptor;
// use bdk::Wallet;
// use bitcoin::hex::FromHex;
// use bdk::bitcoin::Script;
// use std::fs::File;
// use std::io::Write;
// use std::io::{BufRead, BufReader};
// use std::str::FromStr;


// pub fn get_descriptor(script_buf: ScriptBuf)->ScriptBuf{

    
//     let script_hex = script_buf.to_p2sh().to_hex_string();
//     // return "".to_string();

//     // let address: Address<> = Address::new(Network::Regtest, Payload::ScriptHash(script_buf.to_p2sh().script_hash()));
//     let descriptor_str = format!("sh({})", script_hex);

//     let script_bytes = Vec::from_hex(&script_hex).expect("Invalid hex string");
//     let script: &Script = Script::from_bytes(&script_bytes);
//     let p2sh_address = Address::p2sh(script, Network::Regtest).unwrap();
//     println!("{}", p2sh_address);
    
//     let address = Address::from_script(script, Network::Regtest).unwrap();
//     println!("{}", address.script_pubkey());
//     address.script_pubkey()
// }

// pub fn tx_creation(){

//    let descriptor_str = format!("sh({})", "b06dcdcd5bbcb130ae5f1f8e8829cffbfc107e06");
//     let mut wallet = Wallet::new(Descriptor::from_str(&descriptor_str).unwrap(),
//         None, 
//         Network::Regtest,
//         MemoryDatabase::new())
//         .unwrap();
    
    
//     let address = wallet.get_address(bdk::wallet::AddressIndex::New);

// }