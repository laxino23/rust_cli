// https://github.com/rust-cc/awesome-cryptography-rust

#![allow(dead_code)]
use std::io::Read;

use crate::cli::text::TextSignFormat;
use crate::process::gen_pass::process_gen_pass;
use crate::utils::read_input;
use anyhow::Result as aResult;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};

use std::path::Path;

pub trait TextSignable {
    fn sign(&self, reader: &mut dyn Read) -> aResult<Vec<u8>>;
}

pub trait TextVerifiable {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> aResult<bool>;
}

pub trait KeyLoadable: Sized {
    fn load(path: impl AsRef<Path>) -> aResult<Self>;
}

pub trait KeyGenerator {
    fn generate() -> aResult<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl TextSignable for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> aResult<Vec<u8>> {
        // TODO: improve by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerifiable for Blake3 {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> aResult<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == signature)
    }
}
impl KeyGenerator for Blake3 {
    fn generate() -> aResult<Vec<Vec<u8>>> {
        let key = process_gen_pass(32, true, true, true, true)?;
        Ok(vec![key.0.as_bytes().to_vec()])
    }
}

pub struct Ed25519TextSigner {
    key: SigningKey,
}

impl TextSignable for Ed25519TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> aResult<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl KeyGenerator for Ed25519TextSigner {
    fn generate() -> aResult<Vec<Vec<u8>>> {
        let mut csprng = rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        Ok(vec![
            signing_key.to_bytes().to_vec(),
            verifying_key.to_bytes().to_vec(),
        ])
    }
}

pub struct Ed25519TextVerifier {
    key: VerifyingKey,
}

impl TextVerifiable for Ed25519TextVerifier {
    fn verify(&self, reader: &mut dyn Read, signature: &[u8]) -> aResult<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature =
            ed25519_dalek::Signature::from_bytes(signature.try_into()?);
        self.key
            .verify(&buf, &signature)
            .map(|_| true)
            .or_else(|_| Ok(false))
    }
}

pub fn process_sign(
    input: &str,
    key: &str,
    format: TextSignFormat,
) -> aResult<String> {
    let reader = read_input(input, false)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader.as_slice())?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519TextSigner::load(key)?;
            signer.sign(&mut reader.as_slice())?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_verify(
    input: &str,
    key: &str,
    signature: &str,
    format: TextSignFormat,
) -> aResult<bool> {
    let reader = read_input(input, false)?;
    let signature_bytes = URL_SAFE_NO_PAD.decode(signature)?;

    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader.as_slice(), &signature_bytes)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519TextVerifier::load(key)?;
            verifier.verify(&mut reader.as_slice(), &signature_bytes)?
        }
    };
    Ok(verified)
}

pub fn process_key_generate(format: TextSignFormat) -> aResult<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519TextSigner::generate(),
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Blake3 { key }
    }

    fn try_new(key: &[u8]) -> aResult<Self> {
        if key.len() < 32 {
            return Err(anyhow::anyhow!(
                "Key length is too short for Blake3 signing, need at least 32 bytes"
            ));
        }
        let mut key_arr = [0u8; 32];
        key_arr.copy_from_slice(&key[..32]);
        Ok(Blake3 { key: key_arr })
    }
}

impl KeyLoadable for Blake3 {
    fn load(path: impl AsRef<Path>) -> aResult<Self> {
        let key_buf = std::fs::read(path.as_ref())?;
        Self::try_new(&key_buf)
    }
}

impl Ed25519TextSigner {
    fn new(key: SigningKey) -> Self {
        Ed25519TextSigner { key }
    }

    fn try_new(key: &[u8]) -> aResult<Self> {
        if key.len() < 32 {
            return Err(anyhow::anyhow!(
                "Key length is too short for Ed25519 signing, need at least 32 bytes"
            ));
        }
        let mut key_arr = [0u8; 32];
        key_arr.copy_from_slice(&key[..32]);
        Ok(Ed25519TextSigner {
            key: SigningKey::from_bytes(&key_arr),
        })
    }
}

impl KeyLoadable for Ed25519TextVerifier {
    fn load(path: impl AsRef<Path>) -> aResult<Self> {
        let key_buf = std::fs::read(path)?;
        Self::try_new(&key_buf)
    }
}

impl Ed25519TextVerifier {
    fn new(key: VerifyingKey) -> Self {
        Ed25519TextVerifier { key }
    }

    fn try_new(key: &[u8]) -> aResult<Self> {
        if key.len() < 32 {
            return Err(anyhow::anyhow!(
                "Key length is too short for Ed25519 verifying, need at least 32 bytes"
            ));
        }
        let mut key_arr = [0u8; 32];
        key_arr.copy_from_slice(&key[..32]);
        Ok(Ed25519TextVerifier {
            key: VerifyingKey::from_bytes(&key_arr)?,
        })
    }
}

impl KeyLoadable for Ed25519TextSigner {
    fn load(path: impl AsRef<Path>) -> aResult<Self> {
        let key_buf = std::fs::read(path)?;
        Self::try_new(&key_buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_test_verify() -> aResult<()> {
        let key = b"an example very very secret key.";
        let data = b"The quick brown fox jumps over the lazy dog";
        let signer = Blake3::try_new(key)?;
        let signature = signer.sign(&mut data.as_ref())?;
        let verifier = Blake3::try_new(key)?;
        let is_valid = verifier.verify(&mut data.as_ref(), &signature)?;
        assert!(is_valid);
        Ok(())
    }

    #[test]
    fn test_blake3_verify_from_file() -> aResult<()> {
        let vk = Blake3::load("fixtures/blake3.txt")?;
        let data = b"The quick brown fox jumps over the lazy dog";
        let signature = vk.sign(&mut &data[..])?;
        assert!(vk.verify(&mut &data[..], &signature)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> aResult<()> {
        let pk = Ed25519TextSigner::load("fixtures/ed25519_private.key")?;
        let vk = Ed25519TextVerifier::load("fixtures/ed25519_public.key")?;

        let data = b"The quick brown fox jumps over the lazy dog";
        let signature = pk.sign(&mut &data[..])?;
        assert!(vk.verify(&mut &data[..], &signature)?);
        Ok(())
    }
}
