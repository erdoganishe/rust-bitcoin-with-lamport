

use bdk::bitcoin::psbt::{Input, Output, PartiallySignedTransaction};
use bdk::bitcoincore_rpc::RawTx;
use bdk::miniscript::psbt::{PsbtInputExt};
use bdk::miniscript::{DefiniteDescriptorKey, Descriptor};
use bdk::signer::{InputSigner, SignerContext, SignerWrapper};
use bdk::{SignOptions};








use bitcoin::secp256k1::{All, SecretKey};
use bitcoin::{NetworkKind, PrivateKey, PublicKey};
use bitcoin::secp256k1::{Secp256k1};

use bdk::bitcoin::ScriptBuf;

use bdk::bitcoin::{OutPoint, Sequence, Transaction, TxIn, TxOut, Witness};
use hex::{FromHex};
use rand::thread_rng;










// pub fn psbt_creation(script_buf: ScriptBuf, sig: Vec<String>) {
    
//     let secp256k1 = Secp256k1::new();
//     let script_hex = script_buf.to_p2sh().to_hex_string();
//     let script_bytes = Vec::from_hex(&script_hex).expect("Invalid hex string");
//     let script: &Script = Script::from_bytes(&script_bytes);

//     let p2sh_address: Address = Address::p2sh(&script, Network::Regtest).unwrap();

//     println!("P2SH Address: {}", p2sh_address);

//     let mut psbt = Psbt::from_unsigned_tx(Transaction {
//         version: transaction::Version::TWO,
//         lock_time: absolute::LockTime::from_consensus(1257139),
//         input: vec![
//             TxIn {previous_output:OutPoint{
//                 txid:"ce31ffa27c3b6d4e7247387d12f1e971115244728f24b0aa6873a01fa4d5e078".parse().unwrap(),
//                 vout:0,
//             },
//             sequence: Sequence::default(), 
//             script_sig: ScriptBuf::new(),
//             witness: Witness::default(), 
//             }
//         ],
       
//         output: vec![TxOut {
//             script_pubkey: script.into(),
//             value: Amount::from_sat(100000), 
//         }],
//     }).unwrap();


//     psbt.inputs[0].witness_utxo = Some(TxOut {
//         script_pubkey: p2sh_address.script_pubkey(),
//         value: Amount::from_sat(100000), 
//     });

//     // println!("PSBT: {:#?}", psbt);
//     sign_psbt_test(psbt, script_buf, sig, p2sh_address);
// }


fn gen_keys() -> (PrivateKey, PublicKey, Secp256k1<All>) {

    let secp = Secp256k1::new();

    let sk = SecretKey::new(&mut thread_rng());
    let priv_key = PrivateKey::new(sk, NetworkKind::Test);
    let pk = PublicKey::from_private_key(&secp, &priv_key);

    (priv_key, pk, secp)
}

pub fn gen_input_tx(script: ScriptBuf) {
       let mut psbt: PartiallySignedTransaction = PartiallySignedTransaction::from_unsigned_tx(bdk::bitcoin::Transaction {
      
        version: 2,
        lock_time: bdk::bitcoin::absolute::LockTime::from_consensus(1257139),
        input: vec![
            TxIn {previous_output:OutPoint{
                txid:"af8e170648e669fe33a6f366ba523bd17946f7f1911def5b2d59a548685566da".parse().unwrap(),
                vout:0,
            },
            sequence: Sequence::default(), 
            script_sig: ScriptBuf::new(),
            witness: Witness::default(), 
            }
        ],
       
        output: vec![TxOut {
            script_pubkey: script.to_p2sh(),
            value: 100000, 
        }],
    }).unwrap();
    let mut psbt_input = Input{
        sighash_type: None,
        witness_utxo:  
            Some(TxOut {
                script_pubkey: ScriptBuf::from_hex("0014e091ea6aae6adaaea8ee0d5336af169589e0cc98").expect("Valid Script"),
                value: 100000000,
            }),
            ..Default::default() 
    };

    let descriptor_str = "wsh(pk(02a1f1ad0fe384b05504f8233209bad9e396f3f86b591e877dc1f95394306d9b94))";
    let descriptor = descriptor_str.parse::<Descriptor<DefiniteDescriptorKey>>().unwrap();

    psbt_input.update_with_descriptor_unchecked(&descriptor).unwrap();
    psbt.inputs.push(psbt_input);
    psbt.outputs.push(Output::default());
    
    sign_input_tx(psbt);
}
// fn sign_input_tx(psbt: &mut Psbt, script_buf: ScriptBuf) {
//     let (priv_key, pub_key, secp) = gen_keys();
//     let mut key_map = BTreeMap::new();
//     key_map.insert(pub_key, priv_key);

//     let mut map = BTreeMap::new();
//     map.insert(pub_key.inner, (Fingerprint::default(), DerivationPath::default()));
//     psbt.inputs[0].bip32_derivation = map;

//     let txout_unknown_future = TxOut {
//         value: Amount::from_sat(100100),
//         script_pubkey: ScriptBuf::new(),
//     };
//     psbt.inputs[0].witness_utxo = Some(txout_unknown_future);

//     // Sign the PSBT
//     match psbt.sign(&key_map, &secp) {
//         Ok(_) => {
//             let tx: Transaction = psbt.clone().extract_tx().unwrap();
//             let tx_hex = bitcoin::consensus::encode::serialize_hex(&tx);
//             println!("TxId: {}", tx.compute_txid());
//             println!("TxheX : {}", tx_hex);
//         }
//         Err(signing_keys) => {
//             println!("Signing keys error: {:?}", signing_keys);
//         }
//     }
// }

pub fn sign_input_tx(mut psbt: PartiallySignedTransaction){

    let private_key = "cVm5SC4zJYMbz8jHpZkTGQXxbwtyhX76dKb8HVKLnmxS6bbpxVjD";

    let xpriv: bdk::bitcoin::PrivateKey = bdk::bitcoin::PrivateKey::from_wif(private_key).unwrap();
    let signer: SignerWrapper<bdk::bitcoin::PrivateKey> = SignerWrapper::new(
        xpriv,
        SignerContext::Segwitv0
    );
    let sign_options = SignOptions {
        trust_witness_utxo: true,    
        ..Default::default()    
    };
    let tx: Transaction = psbt.clone().extract_tx();
    let tx_hex = tx.raw_hex();
    println!("TxId: {}", tx.txid());
    println!("TxheX : {}", tx_hex);

    let _ = signer.sign_input(& mut psbt, 0, &sign_options, &bdk::bitcoin::secp256k1::Secp256k1::new());
    println!("partial_sigs: {:#?}", psbt.inputs[0].clone().partial_sigs);  
    let tx: Transaction = psbt.extract_tx();
    let tx_hex = tx.raw_hex();
    println!("TxId: {}", tx.txid());
    println!("TxheX : {}", tx_hex);

}

// fn sign_psbt_test(mut psbt2: Psbt, script_buf: ScriptBuf, sig: Vec<String>, address: Address) {

//     let sig_bytes = decode(sig[0].clone()).expect("Error with decoding public key");
//     let mut sig_array: [u8; 32] = [0; 32];
//     if sig_bytes.len() == 32 {
//         sig_array.copy_from_slice(&sig_bytes);
//     } else {
//         panic!("sig[0] does not have exactly 32 bytes");
//     }


//     let unsigned_tx = Transaction {
//         version: transaction::Version::TWO,
//         lock_time: absolute::LockTime::ZERO,
//         input: vec![ TxIn {previous_output:OutPoint{
//             txid:"ce31ffa27c3b6d4e7247387d12f1e971115244728f24b0aa6873a01fa4d5e078".parse().unwrap(),
//             vout:0,
//         },
//         sequence: Sequence::default(), 
//         script_sig: ScriptBuf::new(),
//         witness: Witness::default(), 
//         },
//         TxIn {previous_output:OutPoint{
//             txid:"7d3d198069d5d6e04805bf5d5a9be3a404214242f340ca4a4e76985dedc24714".parse().unwrap(),
//             vout:0,
//         },
//         sequence: Sequence::default(), 
//         script_sig: 
//             ScriptBuf::new(),
//         witness: Witness::default(), 
//         }],
//         output: vec![TxOut{
//             value: Amount::from_sat(2000),
//             script_pubkey: script_buf.clone(),
//         }],
//     };
//     let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();

//     let (priv_key, pk, secp) = gen_keys();

//     let mut key_map = BTreeMap::new();
//     key_map.insert(pk, priv_key);
  
//     psbt.inputs[0].witness_utxo = Some(TxOut {
//         script_pubkey: script_buf.clone(),
//         value: Amount::from_sat(1000), 
//     });
//     psbt.inputs[1].witness_utxo = Some(TxOut {
//         script_pubkey: script_buf.clone(),
//         value: Amount::from_sat(1000), 
//     });
//     let mut map = BTreeMap::new();
//     map.insert(pk.inner, (Fingerprint::default(), DerivationPath::default()));
//     psbt.inputs[0].bip32_derivation = map.clone();
//     psbt.inputs[1].bip32_derivation = map;
//     // Second input is unspendable by us e.g., from another wallet that supports future upgrades.
//     let txout_unknown_future = TxOut {
//         value: Amount::from_sat(1000),
//         script_pubkey:  address.script_pubkey(),
//     };
//     psbt.inputs[0].witness_utxo = Some(txout_unknown_future.clone());
//     psbt.inputs[1].witness_utxo = Some(txout_unknown_future);

//     psbt.inputs[0].final_script_sig =  Some(
//     Builder::new()
//     .push_slice(&sig_array)
//     .into_script());

//     psbt.inputs[1].final_script_sig =  Some(
//         Builder::new()
//         .push_slice(&sig_array)
//         .into_script());

//     let (signing_keys, _) = psbt.sign(&key_map, &secp).unwrap_err();
    
//     println!("psbt edited: {:#?}", psbt);
//     let tx: Transaction = psbt.extract_tx().unwrap();
//     let tx_hex = bitcoin::consensus::encode::serialize_hex(&tx);
//     println!("TxId: {}", tx.compute_txid());
//     println!("TxheX : {}", tx_hex);

    
// }

