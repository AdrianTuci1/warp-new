use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

use anyhow::{Context, Result};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use sha2::{Digest, Sha256};

pub const PAIRING_SALT: &str = "octomus-vps-pairing-v1";

/// Derive a 32-byte key from a pairing code using SHA-256 (PBKDF2 is heavier;
/// for a high-entropy code this is acceptable for the MVP).
pub fn derive_pairing_key(code: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(code.as_bytes());
    hasher.update(PAIRING_SALT.as_bytes());
    hasher.finalize().into()
}

/// Encrypt a plaintext string with the derived key, returning base64(nonce || ciphertext).
pub fn encrypt(key: &[u8; 32], plaintext: &str) -> Result<String> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("encryption failed: {e:?}"))?;
    let mut buf = nonce.to_vec();
    buf.extend_from_slice(&ciphertext);
    Ok(BASE64.encode(&buf))
}

/// Decrypt a base64(nonce || ciphertext) string with the derived key.
pub fn decrypt(key: &[u8; 32], encoded: &str) -> Result<String> {
    let buf = BASE64
        .decode(encoded)
        .context("invalid base64 ciphertext")?;
    if buf.len() < 12 {
        anyhow::bail!("ciphertext too short");
    }
    let (nonce_bytes, ciphertext) = buf.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = ChaCha20Poly1305::new(key.into());
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("decryption failed: {e:?}"))?;
    String::from_utf8(plaintext).context("decrypted payload is not valid utf-8")
}

/// Create a simple pairing challenge: encrypt a known nonce with the key so the
/// server can prove it knows the code without transmitting the code.
pub fn pairing_challenge(key: &[u8; 32]) -> Result<String> {
    encrypt(key, "octomus-vps-challenge")
}

/// Verify that a decrypted challenge matches the expected plaintext.
pub fn verify_pairing_response(key: &[u8; 32], response: &str) -> Result<()> {
    let plaintext = decrypt(key, response)?;
    if plaintext == "octomus-vps-challenge-ack" {
        Ok(())
    } else {
        anyhow::bail!("pairing verification failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let key = derive_pairing_key("test-pairing-code-1234");
        let ciphertext = encrypt(&key, "hello vps").unwrap();
        let plaintext = decrypt(&key, &ciphertext).unwrap();
        assert_eq!(plaintext, "hello vps");
    }

    #[test]
    fn pairing_round_trip() {
        let key = derive_pairing_key("shared-code");
        let challenge = pairing_challenge(&key).unwrap();
        let response = encrypt(&key, "octomus-vps-challenge-ack").unwrap();
        verify_pairing_response(&key, &response).unwrap();
        assert!(
            decrypt(&key, &challenge)
                .unwrap()
                .starts_with("octomus-vps-challenge")
        );
    }
}
