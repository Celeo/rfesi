use base64::engine::{general_purpose::URL_SAFE_NO_PAD as base64, Engine};
use rand::random;
use sha2::{Sha256, Digest};
use crate::errors::EsiResult;

/// PKCEVerifier is a base64urlencoded String to send in the body of the token request alongside the
/// code returned from the ESI and the Client ID
pub type PkceVerifier = String;
pub type PkceChallenge = String;

pub struct Pkce {
    pub challenge: PkceChallenge,
    pub verifier: PkceVerifier,
}

pub fn generate() -> EsiResult<Pkce> {
    let verifier: Vec<u8> = (0..32).map(|_| random()).collect();

    let verifier_base64 = base64url(&verifier);

    // Create SHA256 hash of verifier
    let mut hasher = Sha256::new();
    hasher.update(&verifier_base64);
    let challenge = hasher.finalize();

    // Encode challenge as base64 URL-safe string
    let challenge_base64 = base64url(&challenge.to_vec());
    Ok(Pkce { challenge: challenge_base64, verifier: verifier_base64 })
}

pub fn base64url(verifier: &Vec<u8>) -> String {
    let mut verifier_out = vec![0; verifier.len() * 4 / 3 + 4];
    let written_size = base64.encode_slice(verifier, &mut verifier_out).unwrap();
    verifier_out.truncate(written_size);
    String::from_utf8(verifier_out).unwrap()
}