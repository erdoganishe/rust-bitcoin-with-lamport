
use bdk::bitcoin::address;
use bitcoin::absolute::{self};
use bitcoin::address::Address;
use bitcoin::hashes::Hash;
use bitcoin::psbt::SigningKeys;
use bitcoin::script::Builder;
use bitcoin::secp256k1::{self, ecdsa, All, SecretKey};
use bitcoin::{NetworkKind, PrivateKey, PublicKey};
use bitcoin::secp256k1::{Secp256k1, Message};
use bitcoin::ecdsa::Signature;
use bitcoin::ScriptBuf;
use bitcoin::{transaction, Amount, Network, OutPoint, Psbt, Script, Sequence, Transaction, TxIn, TxOut, Witness};
use hex::{decode, FromHex};
use rand::thread_rng;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::tx;



pub fn psbt_creation(script_buf: ScriptBuf, sig: Vec<String>) {
    
    let secp256k1 = Secp256k1::new();
    let script_hex = script_buf.to_p2sh().to_hex_string();
    let script_bytes = Vec::from_hex(&script_hex).expect("Invalid hex string");
    let script: &Script = Script::from_bytes(&script_bytes);

    let p2sh_address: Address = Address::p2sh(&script, Network::Regtest).unwrap();

    println!("P2SH Address: {}", p2sh_address);

    let mut psbt = Psbt::from_unsigned_tx(Transaction {
        version: transaction::Version::TWO,
        lock_time: absolute::LockTime::from_consensus(1257139),
        input: vec![
            TxIn {previous_output:OutPoint{
                txid:"ce31ffa27c3b6d4e7247387d12f1e971115244728f24b0aa6873a01fa4d5e078".parse().unwrap(),
                vout:0,
            },
            sequence: Sequence::default(), 
            script_sig: ScriptBuf::new(),
            witness: Witness::default(), 
            }
        ],
       
        output: vec![TxOut {
            script_pubkey: script.into(),
            value: Amount::from_sat(100000), 
        }],
    }).unwrap();


    psbt.inputs[0].witness_utxo = Some(TxOut {
        script_pubkey: p2sh_address.script_pubkey(),
        value: Amount::from_sat(100000), 
    });

    // println!("PSBT: {:#?}", psbt);
    sign_psbt_test(psbt, script_buf, sig, p2sh_address);
}


fn gen_keys() -> (PrivateKey, PublicKey, Secp256k1<All>) {

    let secp = Secp256k1::new();

    let sk = SecretKey::new(&mut thread_rng());
    let priv_key = PrivateKey::new(sk, NetworkKind::Test);
    let pk = PublicKey::from_private_key(&secp, &priv_key);

    (priv_key, pk, secp)
}

fn sign_psbt_test(mut psbt2: Psbt, script_buf: ScriptBuf, sig: Vec<String>, address: Address) {
    use bitcoin::bip32::{DerivationPath, Fingerprint};
    use bitcoin::witness_version::WitnessVersion;
    use bitcoin::{WPubkeyHash, WitnessProgram};
    let sig_bytes = decode(sig[0].clone()).expect("Error with decoding public key");
    let mut sig_array: [u8; 32] = [0; 32];
    if sig_bytes.len() == 32 {
        sig_array.copy_from_slice(&sig_bytes);
    } else {
        panic!("sig[0] does not have exactly 32 bytes");
    }


    let unsigned_tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![ TxIn {previous_output:OutPoint{
            txid:"ce31ffa27c3b6d4e7247387d12f1e971115244728f24b0aa6873a01fa4d5e078".parse().unwrap(),
            vout:0,
        },
        sequence: Sequence::default(), 
        script_sig: ScriptBuf::new(),
            // Builder::new()
            // .push_slice(&sig_array)
            // .into_script(),
        witness: Witness::default(), 
        },
        TxIn {previous_output:OutPoint{
            txid:"7d3d198069d5d6e04805bf5d5a9be3a404214242f340ca4a4e76985dedc24714".parse().unwrap(),
            vout:0,
        },
        sequence: Sequence::default(), 
        script_sig: 
            ScriptBuf::new(),
            // Builder::new()
            // .push_slice(&sig_array)
            // .into_script(),
        witness: Witness::default(), 
        }],
        output: vec![TxOut{
            value: Amount::from_sat(2000),
            script_pubkey: script_buf.clone(),
        }],
    };
    let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();

    let (priv_key, pk, secp) = gen_keys();

    let mut file = File::create("keys.txt").expect("Error with file creation");
    writeln!(file, "{:#?},\n\n\n {:#?}", hex::encode(priv_key.to_bytes()), hex::encode(pk.to_bytes())).expect("Error with file filling");

    // key_map implements `GetKey` using KeyRequest::Pubkey. A pubkey key request does not use
    // keysource so we use default `KeySource` (fingreprint and derivation path) below.
    let mut key_map = BTreeMap::new();
    key_map.insert(pk, priv_key);

    // First input we can spend. See comment above on key_map for why we use defaults here.
  
    psbt.inputs[0].witness_utxo = Some(TxOut {
        script_pubkey: script_buf.clone(),
        value: Amount::from_sat(1000), 
    });
    psbt.inputs[1].witness_utxo = Some(TxOut {
        script_pubkey: script_buf.clone(),
        value: Amount::from_sat(1000), 
    });
    let mut map = BTreeMap::new();
    map.insert(pk.inner, (Fingerprint::default(), DerivationPath::default()));
    psbt.inputs[0].bip32_derivation = map.clone();
    psbt.inputs[1].bip32_derivation = map;
    // Second input is unspendable by us e.g., from another wallet that supports future upgrades.
    let txout_unknown_future = TxOut {
        value: Amount::from_sat(1000),
        script_pubkey:  address.script_pubkey(),
    };
    psbt.inputs[0].witness_utxo = Some(txout_unknown_future.clone());
    psbt.inputs[1].witness_utxo = Some(txout_unknown_future);

    psbt.inputs[0].final_script_sig =  Some(
    Builder::new()
    .push_slice(&sig_array)
    .into_script());

    psbt.inputs[1].final_script_sig =  Some(
        Builder::new()
        .push_slice(&sig_array)
        .into_script());

    let (signing_keys, _) = psbt.sign(&key_map, &secp).unwrap_err();
    
    println!("psbt edited: {:#?}", psbt);
    let tx: Transaction = psbt.extract_tx().unwrap();
    let tx_hex = bitcoin::consensus::encode::serialize_hex(&tx);
    println!("TxId: {}", tx.compute_txid());
    println!("TxheX : {}", tx_hex);

    
}