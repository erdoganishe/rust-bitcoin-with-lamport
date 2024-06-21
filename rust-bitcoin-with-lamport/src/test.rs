extern crate bdk;
extern crate bitcoin;

use std::collections::BTreeMap;
use std::default;
use std::str::FromStr;

use bdk::bitcoin::absolute::{LockTime, Time};
use bdk::bitcoin::base58::encode;
use bdk::bitcoin::consensus::encode::{self, serialize};
use bdk::bitcoin::ecdsa::Signature;
use bdk::bitcoin::hashes::hex::FromHex;
use bdk::bitcoin::hashes::{ripemd160, sha256, Hash};
use bdk::bitcoin::psbt::{self, Input, PartiallySignedTransaction as Psbt};
use bdk::bitcoin::{witness, OutPoint, PublicKey, Script, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness};
use bdk::bitcoincore_rpc::RawTx;
use bdk::miniscript::descriptor::{Descriptor, DefiniteDescriptorKey};
use bdk::miniscript::psbt::PsbtInputExt;
use bdk::signer::{InputSigner, SignOptions, SignerContext, SignerWrapper};
use bdk::bitcoin::bip32::{DerivationPath, ExtendedPrivKey, Fingerprint};
use bdk::bitcoin::secp256k1::{ecdsa, Secp256k1};
use bdk::bitcoin::network::constants::Network;
use bdk::bitcoin::key::PrivateKey;
use bitcoin::bip32::Xpriv;

use crate::tx;

pub fn sign_input_tx(mut psbt: Psbt) -> BTreeMap<bdk::bitcoin::PublicKey, bdk::bitcoin::ecdsa::Signature> {
    // Example WIF private key for testnet
    let private_key = "cVm5SC4zJYMbz8jHpZkTGQXxbwtyhX76dKb8HVKLnmxS6bbpxVjD";
   
    let private_key_from_str = PrivateKey::from_wif(private_key).expect("Invalid WIF");


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



    display_psbt(psbt.clone());

    let res = psbt.inputs[0].clone().partial_sigs;

    res

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

    let pub_key: bdk::bitcoin::secp256k1::PublicKey = bdk::bitcoin::secp256k1::PublicKey::from_str("02a1f1ad0fe384b05504f8233209bad9e396f3f86b591e877dc1f95394306d9b94").unwrap();
    let sha256_hash = sha256::Hash::hash(&pub_key.clone().serialize());
    let ripemd160_hash = ripemd160::Hash::hash(&sha256_hash.to_byte_array());
    let fingerprint_bytes = &ripemd160_hash[..4];

    let fingerprint = Fingerprint::from_str(&hex::encode(&fingerprint_bytes)).unwrap();
    let der_path = DerivationPath::default();

    // println!("________________________________________{}", fingerprint);
    let mut derivation = BTreeMap::new();
    derivation.insert(pub_key, (fingerprint,der_path));
    psbt_input.bip32_derivation = {
        derivation
    };

    psbt_input.final_script_witness = Some(witness);
    //psbt_input.partial_sigs = map;
    psbt_input.final_script_sig = Some(ScriptBuf::new());
    psbt.inputs[0] = psbt_input.clone();

    println!("{:#?}", psbt);


    display_psbt(psbt.clone());
    
    // let tx_2 = deserialize_tx_hex("020000000001013b97513701fefb56c0e7b3a721db6846769810a31d725e4fff47f1a8b959b3cc0b00000000fdffffff02066f0000000000001600142aacff93730a2a8042575dc8704e31a39922889e00000000000000000a6a5d0714c2a23314b5170247304402206d08686cace0693336c1bf0521f17384fed319b6be7e995c118decf03e000ccf022063086439c3046bd56dec03d2ffb9b4f43f5abbf14ea019236a5d14e440c2a133012103b47bbc7cd630633ac62c86fe472944aa322260d822f06b65847175f80425961a00000000");
    // let psbt_2 = Psbt::from_unsigned_tx(tx_2).unwrap();
    // println!("{:#?}", psbt_2.inputs[0]);


}

fn deserialize_tx_hex(tx_hex: &str) -> Transaction {
    let tx_bytes = Vec::from_hex(tx_hex).expect("Invalid hex string");
    encode::deserialize(&tx_bytes).expect("Failed to deserialize transaction")
}


fn create_witness(sig: &Signature, pubkey: &PublicKey) -> Witness {
    let mut witness = Witness::new();

    let sig_str = sig.sig.to_string();
    let sig_type = sig.hash_ty;

    let ser_sig: ecdsa::SerializedSignature = ecdsa::SerializedSignature::from_signature(& sig.sig);
    witness.push_bitcoin_signature(&ser_sig, bdk::bitcoin::sighash::EcdsaSighashType::All);
    witness    
}


pub fn display_psbt(psbt: Psbt){
    let tx: Transaction = psbt.extract_tx();
    let tx_hex = tx.raw_hex();
    println!("TxId: {}", tx.txid());
    println!("TxheX : {}", tx_hex);

}