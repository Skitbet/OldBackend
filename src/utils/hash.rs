use sha2::{Digest, Sha256};

/// Computes the full SHA-256 hash of the input bytes as a lowercase hex string.
pub fn hash_bytes_sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Computes a shortened SHA-256 hash (first 10 hex chars) of the input bytes.
pub fn hash_bytes_sha256_slimmed(bytes: &[u8]) -> String {
    let full_hash = hash_bytes_sha256(bytes);
    full_hash[..10].to_string()
}
