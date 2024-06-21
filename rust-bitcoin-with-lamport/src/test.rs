use anyhow::{anyhow, Context, Result};
use bitcoin::psbt::Input;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::{All, Message, Secp256k1};
use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey};
use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::util::sighash::SighashCache;
use bitcoin::{EcdsaSig, EcdsaSighashType, PublicKey};

fn set_sighash_type(signature: Signature, input: &Input) -> EcdsaSig {
    let sighash_type = get_sighash_type(input);
    EcdsaSig {
        sig: signature,
        hash_ty: sighash_type,
    }
}

fn get_sighash_type(input: &Input) -> EcdsaSighashType {
    input
        .sighash_type
        .and_then(|t| t.ecdsa_hash_ty().ok())
        .unwrap_or(EcdsaSighashType::All)
}

fn get_partial_derivation(
    derivation: &DerivationPath,
    sub_derivation: &DerivationPath,
) -> Result<DerivationPath> {
    if derivation.len() > sub_derivation.len() {
        return Err(anyhow!(
            "Can't get a partial derivation from a derivation greater than the sub derivation"
        ));
    }
    let partial = &sub_derivation[derivation.len()..];
    dbg!(&partial);
    DerivationPath::try_from(partial).map_err(|e| anyhow!("{e}"))
}

fn derive_relative_xpriv(
    xprv: &ExtendedPrivKey,
    secp: &Secp256k1<All>,
    derivation: &DerivationPath,
    sub_derivation: &DerivationPath,
) -> Result<ExtendedPrivKey> {
    xprv.derive_priv(secp, &get_partial_derivation(derivation, sub_derivation)?)
        .map_err(|e| anyhow!("{e}"))
}

fn sign_psbt(
    mut psbt: PartiallySignedTransaction,
    xprv: ExtendedPrivKey,
    derivation: &DerivationPath,
) -> Result<PartiallySignedTransaction> {
    let secp = Secp256k1::new();    
    for (index, input) in psbt.inputs.iter_mut().enumerate() {
        let witness_script = input
            .witness_script
            .as_ref()
            .context("Missing witness script")?;
        let amount = input
            .witness_utxo
            .as_ref()
            .context("Witness utxo not found")?
            .value;        
        let mut sighash_cache = SighashCache::new(&psbt.unsigned_tx);
        let sighash = sighash_cache.segwit_signature_hash(
            index,
            witness_script,
            amount,
            get_sighash_type(input),
        )?;        
        let mut input_keypairs = Vec::new();        
        for (_, (fingerprint, sub_derivation)) in input.bip32_derivation.iter() {
            if fingerprint != &xprv.fingerprint(&secp) {
                continue;
            }
            let parent_xprv = derive_relative_xpriv(&xprv, &secp, derivation, sub_derivation)?;
            input_keypairs.push(parent_xprv.to_keypair(&secp));
        }        if input_keypairs.is_empty() {
            return Err(anyhow!("No private keys to sign this psbt"));
        }        for keypair in input_keypairs {
            let message = &Message::from_slice(&sighash)?;
            let signature = secp.sign_ecdsa(message, &keypair.secret_key());
            input.partial_sigs.insert(
                PublicKey::new(keypair.public_key()),
                set_sighash_type(signature, input),
            );            secp.verify_ecdsa(message, &signature, &keypair.public_key())?;
        }
    }    Ok(psbt)
}