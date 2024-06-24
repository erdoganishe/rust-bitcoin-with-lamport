extern crate bdk;
extern crate bitcoin;

use std::collections::BTreeMap;
use std::str::FromStr;
use bdk::bitcoin::absolute::LockTime;
use bdk::bitcoin::consensus::encode::{self};
use bdk::bitcoin::ecdsa::Signature;
use bdk::bitcoin::hashes::hex::FromHex;
use bdk::bitcoin::hashes::{ripemd160, sha256, Hash};
use bdk::bitcoin::psbt::{Input, PartiallySignedTransaction as Psbt};
use bdk::bitcoin::script::PushBytesBuf;
use bdk::bitcoin::{OutPoint, PublicKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness};
use bdk::bitcoincore_rpc::RawTx;
use bdk::miniscript::descriptor::{Descriptor, DefiniteDescriptorKey};
use bdk::miniscript::psbt::PsbtInputExt;
use bdk::signer::{InputSigner, SignOptions, SignerContext, SignerWrapper};
use bdk::bitcoin::bip32::{DerivationPath, Fingerprint};
use bdk::bitcoin::secp256k1::{ecdsa, Secp256k1};

use bdk::bitcoin::key::PrivateKey;




pub fn sign_input_tx(mut psbt: Psbt) -> BTreeMap<bdk::bitcoin::PublicKey, bdk::bitcoin::ecdsa::Signature> {
    // Example WIF private key for testnet
    let private_key = "cVm5SC4zJYMbz8jHpZkTGQXxbwtyhX76dKb8HVKLnmxS6bbpxVjD";
   
    let _private_key_from_str = PrivateKey::from_wif(private_key).expect("Invalid WIF");


    let xpriv: PrivateKey = PrivateKey::from_wif(private_key).unwrap();

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

    

    psbt.inputs[0].clone().partial_sigs

}

pub fn gen_input_tx_test() {
    let script = ScriptBuf::from_hex("ebf5d1c9c52233e0693085337f88b0515d823ab2").unwrap();
    
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
                script_pubkey: script.to_p2sh(),
            }
        ],
    };

    let mut psbt = Psbt::from_unsigned_tx(unsigned_tx).unwrap();
    
    let mut psbt_input = Input {
        witness_utxo: Some(TxOut {
            value: 100000000,
            script_pubkey: ScriptBuf::from_hex("0014e091ea6aae6adaaea8ee0d5336af169589e0cc98").unwrap(),
            //Не впливає на хекс
        
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

    let fingerprint = Fingerprint::from_str(&hex::encode(fingerprint_bytes)).unwrap();
    let der_path = DerivationPath::default();

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

}


fn create_witness(sig: &Signature, pubkey: &PublicKey) -> Witness {
    let mut witness = Witness::default();

    let ser_sig: ecdsa::SerializedSignature = ecdsa::SerializedSignature::from_signature(& sig.sig);
    witness.push_bitcoin_signature(&ser_sig, bdk::bitcoin::sighash::EcdsaSighashType::All);
    let mut data = PushBytesBuf::new();
    let _ = data.extend_from_slice(&pubkey.inner.serialize());
    witness.push(data.as_push_bytes());
    
    witness    
}


pub fn display_psbt(psbt: Psbt){
    let tx: Transaction = psbt.extract_tx();
    let tx_hex = tx.raw_hex();
    println!("TxId: {}", tx.txid());
    println!("TxheX : {}", tx_hex);

}