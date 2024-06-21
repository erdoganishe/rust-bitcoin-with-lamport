extern crate bdk;
extern crate bitcoin;

use std::collections::BTreeMap;
use std::default;

use bdk::bitcoin::absolute::{LockTime, Time};
use bdk::bitcoin::consensus::encode::serialize;
use bdk::bitcoin::ecdsa::Signature;
use bdk::bitcoin::psbt::{PartiallySignedTransaction as Psbt, Input};
use bdk::bitcoin::{witness, OutPoint, PublicKey, Script, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness};
use bdk::bitcoincore_rpc::RawTx;
use bdk::miniscript::descriptor::{Descriptor, DefiniteDescriptorKey};
use bdk::miniscript::psbt::PsbtInputExt;
use bdk::signer::{InputSigner, SignOptions, SignerContext, SignerWrapper};
use bdk::bitcoin::bip32::{ExtendedPrivKey, DerivationPath};
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::network::constants::Network;
use bdk::bitcoin::key::PrivateKey;

pub fn sign_input_tx(mut psbt: Psbt) -> BTreeMap<bdk::bitcoin::PublicKey, bdk::bitcoin::ecdsa::Signature> {
    // Example WIF private key for testnet
    let private_key = "cVm5SC4zJYMbz8jHpZkTGQXxbwtyhX76dKb8HVKLnmxS6bbpxVjD";
    
    let xpriv: PrivateKey = PrivateKey::from_wif(&private_key).unwrap();
    let signer: SignerWrapper<PrivateKey> = SignerWrapper::new(
        xpriv,
        SignerContext::Segwitv0,
    );

    let sign_options = SignOptions {
        trust_witness_utxo: true,
        ..Default::default()
    };

    let secp = Secp256k1::new();
    let _ = signer.sign_input(&mut psbt, 0, &sign_options, &secp);
    println!("partial_sigs: {:#?}", psbt.inputs[0].partial_sigs);



    let tx: Transaction = psbt.clone().extract_tx();
    let tx_hex = tx.raw_hex();
    println!("TxId: {}", tx.txid());
    println!("TxheX : {}", tx_hex);

    let res = psbt.inputs[0].clone().partial_sigs;

    return res

}

pub fn gen_input_tx_test() {
    let script = ScriptBuf::from_hex("76a914e091ea6aae6adaaea8ee0d5336af169589e0cc9888ac").unwrap();
    
    let unsigned_tx = Transaction {
        version: 2,
        lock_time: LockTime::from_time(1653195600).unwrap(),
        input: vec![
            TxIn {
                previous_output: OutPoint {
                    txid: "af8e170648e669fe33a6f366ba523bd17946f7f1911def5b2d59a548685566da".parse().unwrap(),
                    vout: 0,
                },
                sequence: Sequence::default(),
                script_sig: ScriptBuf::new(),
                witness: Witness::default(),
            }
        ],
        output: vec![
            TxOut {
                value: 100000,
                script_pubkey: script,
            }
        ],
    };

    let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();
    
    let mut psbt_input = Input {
        witness_utxo: Some(TxOut {
            value: 100000000,
            script_pubkey: ScriptBuf::from_hex("0014e091ea6aae6adaaea8ee0d5336af169589e0cc98").unwrap(),
        }),
        ..Default::default()
    };

    let descriptor_str = "wsh(pk(02a1f1ad0fe384b05504f8233209bad9e396f3f86b591e877dc1f95394306d9b94))";
    let descriptor = descriptor_str.parse::<Descriptor<DefiniteDescriptorKey>>().unwrap();

    psbt.inputs[0] = psbt_input.clone();
    psbt.inputs[0].update_with_descriptor_unchecked(&descriptor).unwrap();

    let mut map = sign_input_tx(psbt.clone());
    let first = map.first_entry().expect("map should have at least one element");
    let pk = first.key();
    let sig = first.get();
    let witness = create_witness(sig, pk);
    println!("Pk: {}\nSig {}", pk, sig);
    println!("Withness : {:#?}", witness);
    psbt_input.final_script_witness = Some(witness);

    psbt.inputs[0] = psbt_input.clone();
    display_psbt(psbt.clone());
    
    
}


fn create_witness(sig: &Signature, pubkey: &PublicKey) -> Witness {
    let mut witness = Witness::new();
    witness.push(sig.to_vec());
    witness.push(pubkey.to_bytes());

    witness    
}


pub fn display_psbt(psbt: Psbt){
    let tx: Transaction = psbt.extract_tx();
    let tx_hex = tx.raw_hex();
    println!("TxId: {}", tx.txid());
    println!("TxheX : {}", tx_hex);

}